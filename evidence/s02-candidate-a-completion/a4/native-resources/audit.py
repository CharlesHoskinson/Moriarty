"""Root intake of original RH002 short-process evidence; not native acceptance."""
import ast
import hashlib
import json
from pathlib import Path
import re

ROOT=Path.cwd()
BASE=ROOT/'.superpowers/sdd/rh002-unit-receipts'
def sha(path):
    h=hashlib.sha256()
    with Path(path).open('rb') as stream:
        while block:=stream.read(1048576):h.update(block)
    return h.hexdigest()
def read(path):return json.loads(path.read_bytes())
index_path=ROOT/'.superpowers/sdd/rh002-unit-artifact-index.json'
report=ROOT/'.superpowers/sdd/rh002-unit-report.md'
assert sha(index_path)=='5946af02da299d3f5813c6086908730a73cde8168693d5135deb9f43b14ef8f6'
assert sha(report)=='46acc51d07c4e26f0579154688fd3da5dabb4ac6dfc45ad61d9070b661045ca8'
index=read(index_path)
assert len(index['files'])==215 and len(index['symlinks'])==25
for name,pin in index['files'].items():
    path=ROOT/name
    assert path.is_file() and not path.is_symlink() and path.resolve().is_relative_to(ROOT)
    assert path.stat().st_size==pin['bytes'] and sha(path)==pin['sha256']
for name,target in index['symlinks'].items():
    assert (ROOT/name).is_symlink() and str((ROOT/name).readlink())==target
runner='evidence/s02-candidate-a-completion/a4/native-resources/runner.py'
test='evidence/s02-candidate-a-completion/a4/native-resources/test_runner.py'
assert sha(ROOT/runner)=='d8973d3541269d1be2ce524f2482e5f5b858f15f7d9117e3fe6d5cd2171d660d'
assert sha(ROOT/test)=='e1e81c5105a8872b20a526a6f9e8bf1add1c498ba31e65f5019938a9a0986dc3'
runtime=None; stages={}; members=0
for name,code in [('red',1),('green',0)]:
    folder=BASE/name
    launch,result=read(folder/'launch.json'),read(folder/'terminal.json')
    assert result['exit_code']==result['recorder_exit_code']==code
    assert result['source_stable'] and result['runtime_stable']
    assert result['source_before']==result['source_after'] and len(result['source_before'])==5
    assert result['runtime_before']==result['runtime_after']
    if runtime is None:runtime=result['runtime_before']
    assert runtime==result['runtime_before'] and len(runtime['files'])==8089
    assert result['elapsed_ns']==result['ended_ns']-result['started_ns']>0
    assert launch['before_dispatch_base']=='3a7d32d3a812207e860e340851aa17c1a8b3494a'
    assert launch['cwd']==str(ROOT)
    parent=['/home/charl/Moriarty/.venv/bin/python','-B','-X',
        'pycache_prefix=.superpowers/sdd/rh002-unit-receipts/'+name+'/python-cache',
        '.superpowers/sdd/rh002-unit/record.py',name]
    child=['/home/charl/Moriarty/.venv/bin/python','-B','-X',
        'pycache_prefix='+str(folder/'python-cache/child'),'-m','pytest',test,'-vv',
        '--basetemp='+str(folder/'basetemp')]
    if code:parent+=['-k','present_empty_resource'];child+=['-k','present_empty_resource']
    assert launch['original_parent_argv']==parent and launch['command']==result['command']==child
    for when in ('before','after'):
        paths={str(p.relative_to(folder/when)) for p in (folder/when).rglob('*') if p.is_file()}
        assert paths==set(result['source_'+when])
        for path,digest in result['source_'+when].items():
            assert sha(folder/when/path)==digest
            members+=1
            if not code:assert sha(ROOT/path)==digest
    for stream in ('stdout','stderr'):assert sha(folder/(stream+'.bin'))==result[stream+'_sha256']
    assert (folder/'stderr.bin').stat().st_size==0
    stages[name]={'exitCode':code,'elapsedNanoseconds':result['elapsed_ns'],
                  'launchSha256':sha(folder/'launch.json'),'terminalSha256':sha(folder/'terminal.json')}
assert 'AssertionError: assert True is False' in (BASE/'red/stdout.bin').read_text()
assert '1 failed' in (BASE/'red/stdout.bin').read_text()
green_text=(BASE/'green/stdout.bin').read_text()
nodes=re.findall(r'(?m)^(evidence/\S+) PASSED ',green_text)
assert len(nodes)==len(set(nodes))==41 and '41 passed' in green_text and 'deselected' not in green_text
red_tests=(BASE/'red/before'/test).read_text()
assert (ROOT/test).read_text().startswith(red_tests)
red_function=next(n for n in ast.parse((BASE/'red/before'/runner).read_text()).body
                  if isinstance(n,ast.FunctionDef) and n.name=='resource_complete')
assert len(red_function.body)==1 and ast.dump(red_function.body[0])==ast.dump(ast.parse('return Path(path).is_file()').body[0])
for path,digest in runtime['files'].items():assert sha(path)==digest
for directory,names in runtime['tree_members'].items():
    assert sorted({str(p.resolve()) for p in Path(directory).rglob('*') if p.is_file()})==names
assert runtime['verification']=={
    'beforeDispatchCommit':'900bb2051225b4a3d99bf422c3b2e5e386e3e7bc',
    'dispatchSha256':'393a8aa904bcd9473670ac48934b3dca4ed611ca9b9accbfa94f15bb4798ac2f',
    'runtimeArchiveSha256':'f0687656d30c0d59ece8dfd027508a9228020506d29b97aea3abe7568186c31c',
    'runtimeManifestSha256':'fa3e9838c66480557ed6d928a2d159ebc074d05f67f15ef08ab20388d9c49e0b'}
captures={}
for terminal in sorted((BASE/'green/basetemp').glob('*/capture/terminal.json')):
    # glob does not recurse through aliases, but the alias itself can match.
    if terminal.parent.parent.is_symlink():continue
    r=read(terminal); folder=terminal.parent; launch=read(folder/'launch.json')
    assert r['inner_receipt_complete'] is False and r['inner_receipt_independent_validation_required'] is True
    assert r['wall_seconds']==launch['wall_seconds'] and r['elapsed_seconds']>0
    assert launch['argv']==['/usr/bin/time','-v','-o',str(folder/'resources.txt'),*launch['child_argv']]
    assert launch['fixed_environment']=={'PYTHONNOUSERSITE':'1','PYTHONDONTWRITEBYTECODE':'1','NODE_DISABLE_COMPILE_CACHE':'1','LC_ALL':'C'}
    for path,digest in r['sidecar_sha256'].items():
        assert sha(path)==digest if digest is not None else not Path(path).is_file()
    assert r['source_runtime_before']==launch['source_runtime_before']
    assert sha(folder/'runner.py')==sha(ROOT/runner)
    assert r['source_runtime_before'][str(ROOT/runner)]==sha(ROOT/runner)
    if r['inner_receipt_present']:
        assert sha(launch['inner_receipt'])==r['inner_receipt_sha256']
    else:assert r['inner_receipt_sha256'] is None
    if r['resource'] is not None:
        raw=(folder/'resources.txt').read_text()
        fields={};diagnostics=[]
        for line in raw.splitlines():
            if line.startswith('\t'):
                key,value=line[1:].split(': ',1);assert key not in fields;fields[key]=value
            else:diagnostics.append(line)
        assert len(fields)==23 and fields==r['resource']['fields']
        assert fields['Command being timed']=='"'+' '.join(launch['child_argv'])+'"'
        assert int(fields['Maximum resident set size (kbytes)'])==r['resource']['maximum_rss_kib']>0
        assert int(fields['Exit status'])==r['resource']['exit_status']
        assert diagnostics==r['resource']['diagnostics']
    if r['eligible']:
        assert r['return_code']==r['actual_exit']==0 and r['resource_complete']
        assert r['source_runtime_stable'] and r['cleanup']=={'forced':False,'signals':[],'errors':[],'complete':True}
        assert not r['timed_out'] and r['inner_receipt_present']
    if r['timed_out']:assert r['return_code']==124 and not r['resource_complete'] and not r['eligible']
    if r['cleanup']['forced']:assert not r['eligible']
    captures[folder.parent.name]={'actualExit':r['actual_exit'],'returnCode':r['return_code'],
        'eligible':r['eligible'],'timedOut':r['timed_out'],'forcedCleanup':r['cleanup']['forced'],
        'cleanupComplete':r['cleanup']['complete'],'terminalSha256':sha(terminal)}
assert len(captures)==21
for suffix,code in [('0',124),('1',2)]:
    folder=BASE/'green/basetemp'/('test_leader_exit_never_hides_l'+suffix)
    r=read(folder/'capture/terminal.json')
    assert r['return_code']==code and r['cleanup']['forced'] and not r['eligible']
    assert {'signal':'SIGKILL','sent':True} in r['cleanup']['signals']
    assert r['cleanup']['complete']  # Observed originals; not assumed for other kernels.
assert captures['test_nonzero_child_preserves_a0']['returnCode']==7
assert captures['test_missing_inner_is_not_elig0']['returnCode']==2
assert captures['test_real_launch_error_retains0']['returnCode']==2
print(json.dumps({'ok':True,'scope':'RH002-local-short-process-unit-only',
    'stages':stages,'originalSourceMembers':members,'runtimePins':8089,'originalFiles':215,
    'originalSymlinks':25,'pythonTests':41,'orderedPassingNodeIds':nodes,
    'actualCaptureRecords':21,'captures':captures},indent=2))
