"""Root intake of all original Task2 terminal evidence, not a model-checking pass."""
import ast
import hashlib
import json
from pathlib import Path
import re
import shlex
import tarfile
import xml.etree.ElementTree as ET

ROOT=Path.cwd()
R=ROOT/'.superpowers/sdd/a5-factoring-receipts'
MODEL=ROOT/'specs/quint/s02/factored_verification'
def stream_sha(stream):
    h=hashlib.sha256()
    while block:=stream.read(1048576):h.update(block)
    return h.hexdigest()
def sha(path):
    with Path(path).open('rb') as stream:return stream_sha(stream)
def read(path):return json.loads(path.read_bytes())
index=read(R/'task2-terminal-index.json')
assert sha(R/'task2-terminal-index.json')=='7db8df0bc6079fec9fcda16fef9c0b45fab887af963f4a81cecb59d631d1e1d7'
assert sha(ROOT/'.superpowers/sdd/a5-corpus-task2-report.md')=='e035e32986e2cbc19d54aef1055f2994be46949fc23c1629be19e1ed5b8a9322'
assert index['commandCount']==len(index['stages'])==52
assert len({row['stage'] for row in index['stages']})==52
assert index['actualTask2Base']=='d824fa3381ecda846be7e8dd027732f1a7f62e91'
assert index['runtimeBootstrapBase']=='900bb2051225b4a3d99bf422c3b2e5e386e3e7bc'
frozen=read(R/'task2-frozen-source.json')
assert index['frozenSources']==frozen and len(frozen['files'])==30
for entry in frozen['files']:assert sha(ROOT/entry['path'])==entry['sha256']
stage_names={row['stage'] for row in index['stages']}
originals=read(MODEL/'derivation.json')['originals']
corpus=[row for row in originals if row['runs']]
expected_corpus={f'corpus-{lane}-{Path(row["path"]).stem}-{op}'
    for lane in ('original','factored') for row in corpus for op in ('typecheck','tests')}
extra={'boundary-red-typecheck','boundary-red-test',
    'candidate_a_factoring_equivalence_test-typecheck','candidate_a_factoring_equivalence_test-tests',
    'task2-installment-sample-100','task2-swap-sample-100','task2-original-python-collect',
    'task2-original-python-regression','task2-original-python-uv-collect',
    'task2-original-python-uv-alias-collect','task2-original-python-uv-alias-regression','core-comparison-uv-alias'}
assert stage_names==expected_corpus|extra
failures={'boundary-red-test','task2-original-python-regression','task2-original-python-uv-collect'}
members=0; originals_checked=0; stages={}
for entry in index['stages']:
    name=entry['stage'];stage=R/name
    for e in entry['files']:
        path=ROOT/e['path']
        assert path.is_file() and not path.is_symlink() and path.resolve().is_relative_to(ROOT)
        assert path.stat().st_size==e['bytes'] and sha(path)==e['sha256']
        originals_checked+=1
    inp,result=read(stage/'input.json'),read(stage/'result.json')
    assert result==entry['result'] and result['exitCode']==entry['expectedExitCode']==int(name in failures)
    assert not result['timedOut'] and result['sourceAndToolsUnchanged'] and result['runtimeUnchanged']
    assert inp['sourceCommit']==entry['sourceCommit']
    assert inp['beforeDispatchCommit']==entry['beforeDispatchCommit']==index['runtimeBootstrapBase']
    assert inp['cwd']==str(ROOT)
    assert inp['limits']=={'wallSeconds':1200,'jvmHeapMiB':4096,'nodeHeapMiB':4096}
    assert 0<result['durationSeconds']<1200
    assert inp['runtimeSnapshot']['runtimeManifestSha256']=='fa3e9838c66480557ed6d928a2d159ebc074d05f67f15ef08ab20388d9c49e0b'
    assert inp['runtimeSnapshot']['runtimeArchiveSha256']=='f0687656d30c0d59ece8dfd027508a9228020506d29b97aea3abe7568186c31c'
    assert len(inp['pins'])==len({pin['path'] for pin in inp['pins']})
    local={str(Path(p['path']).relative_to(ROOT)):p['sha256'] for p in inp['pins'] if Path(p['path']).is_relative_to(ROOT)}
    assert sha(stage/'source-and-runner.tar.gz')==inp['sourceArchiveSha256']
    with tarfile.open(stage/'source-and-runner.tar.gz') as archive:
        items=archive.getmembers()
        assert len(items)==entry['archiveMemberCount']==len(local)
        assert {m.name for m in items}==set(local) and all(m.isfile() for m in items)
        for item in items:
            assert stream_sha(archive.extractfile(item))==local[item.name]
            if not (name.startswith('boundary-red-') and item.name.endswith('candidate_a_authority_boundary_f.qnt')):
                assert sha(ROOT/item.name)==local[item.name]
            members+=1
    for p in inp['pins']:
        if not Path(p['path']).is_relative_to(ROOT):assert sha(p['path'])==p['sha256']
    for stream in ('stdout','stderr'):assert sha(stage/(stream+'.txt'))==result[stream+'Sha256']
    assert re.search(r'Exit status: '+str(result['exitCode'])+r'\b',(stage/'resources.txt').read_text())
    stages[name]={'inputSha256':sha(stage/'input.json'),'resultSha256':sha(stage/'result.json'),
                  'sourceMembers':len(local),'exitCode':result['exitCode']}
for e in index['attachments']:
    p=ROOT/e['path'];assert p.stat().st_size==e['bytes'] and sha(p)==e['sha256']

counts={}
for lane in ('original','factored'):
    counts[lane]=0
    for row in corpus:
        base=Path(row['path']);stem=base.stem
        entry=ROOT/base if lane=='original' else MODEL/(stem+'_f.qnt')
        for operation in ('typecheck','tests'):
            stage=R/f'corpus-{lane}-{stem}-{operation}'
            inp=read(stage/'input.json')
            tail=['typecheck',str(entry)] if operation=='typecheck' else ['test',str(entry),'--backend=rust','--seed=42','--match','.*Test']
            assert inp['argv'][2:]==tail
            if operation=='tests':
                text=(stage/'stdout.txt').read_text()
                assert re.findall(r'ok (\w+) passed \d+ test\(s\)',text)==row['runs']
                assert re.findall(r'(?m)^\s*(\d+) passing\b',text)==[str(len(row['runs']))]
                counts[lane]+=len(row['runs'])
assert counts=={'original':244,'factored':244}
eq_names=re.findall(r'(?m)^\s*run (\w+) =',(MODEL/'candidate_a_factoring_equivalence_test.qnt').read_text())
assert len(eq_names)==6
eq=R/'candidate_a_factoring_equivalence_test-tests'
assert re.findall(r'ok (\w+) passed \d+ test\(s\)',(eq/'stdout.txt').read_text())==eq_names
assert read(eq/'input.json')['argv'][2:]==['test',str(MODEL/'candidate_a_factoring_equivalence_test.qnt'),'--backend=rust','--seed=42','--match','.*Test']
witness_counts={}
for life,n in [('installment',25),('swap',24)]:
    original=read(ROOT/('.superpowers/sdd/a2-task3-receipts/samples-100/input.json' if life=='installment' else '.superpowers/sdd/a3-task3-receipts/green/samples100.json'))
    command=original['argv'] if life=='installment' else shlex.split(original['command'])
    command[0]='/home/charl/.npm-global/bin/quint'
    command[2]=str(MODEL/(Path(command[2]).stem+'_f.qnt'))
    stage=R/f'task2-{life}-sample-100'; inp=read(stage/'input.json')
    assert inp['requestedArgv']==command
    assert inp['argv'][2:]==command[1:]
    assert '--max-samples=100' in command and '--max-steps='+str(inp['depth']) in command
    expected=command[command.index('--witnesses')+1:command.index('--verbosity=1')]
    rows=re.findall(r'(?m)^(\w+) was witnessed in (\d+) trace\(s\) out of (\d+) explored', (stage/'stdout.txt').read_text())
    assert [r[0] for r in rows]==expected and len(rows)==n
    assert all(0<int(hit)<=100 and total=='100' for _,hit,total in rows)
    assert '[ok] No violation found' in (stage/'stdout.txt').read_text()
    witness_counts[life]={name:int(hit) for name,hit,_ in rows}

fixture=read(R/'task2-python-inputs/manifest.json')
assert len(fixture['files'])==2691 and len(fixture['originalTests'])==23
assert sha(R/'task2-python-inputs/manifest.json')=='7e8e5297a333dd91cd5aef8a9aa755355b2941b62cdd4aad185f95a86a5fa04c'
assert sha(R/'task2-python-inputs/source-inputs.tar.gz')==fixture['archiveSha256']=='e0bbb3ca31de72834921cce6e30fc225bceeef478be6b5a7bfe7b50d437f240a'
with tarfile.open(R/'task2-python-inputs/source-inputs.tar.gz') as archive:
    entries={e['archivePath']:e for e in fixture['files']};items=archive.getmembers()
    assert len(items)==2691 and {m.name for m in items}==set(entries)
    for member in items:
        e=entries[member.name]
        assert member.isfile() and member.size==e['bytes']
        assert stream_sha(archive.extractfile(member))==e['sha256']==sha(e['path'])
ids=read(R/'task2-original-python-collect/collected-nodeids.json')
fresh=read(R/'task2-original-python-uv-alias-collect/collected-nodeids.json')
assert len(ids)==len(set(ids))==441 and fresh==ids
passed=R/'task2-original-python-uv-alias-regression'
document=ET.parse(passed/'junit.xml'); actual=[]
for case in document.findall('.//testcase'):
    assert all(case.find(tag) is None for tag in ('failure','error','skipped'))
    classname,name=case.attrib['classname'],case.attrib['name']
    matches=[p for p in fixture['originalTests'] if classname==p[:-3].replace('/','.') or classname.startswith(p[:-3].replace('/','.')+'.')]
    assert len(matches)==1
    p=matches[0]; suffix=classname[len(p[:-3].replace('/','.')):].lstrip('.')
    actual.append(p+'::'+(suffix.replace('.','::')+'::' if suffix else '')+name)
assert len(actual)==len(set(actual))==441 and set(actual)==set(ids)
assert sha(passed/'junit.xml')=='978c7fbc7833bb32a5c27a901b19b3088d264e207a707df129ef5869f7652d5b'
assert '441 passed' in (passed/'stdout.txt').read_text()
for name in ('task2-original-python-collect','task2-original-python-regression','task2-original-python-uv-collect',
             'task2-original-python-uv-alias-collect','task2-original-python-uv-alias-regression','core-comparison-uv-alias'):
    stage=R/name; checks=read(stage/'fixture-checks.json'); inp=read(stage/'input.json')
    assert checks['unchangedBefore'] and checks['unchangedAfter'] and checks['commandExitCode']==stages[name]['exitCode']
    assert checks['manifestSha256']==sha(R/'task2-python-inputs/manifest.json')
    assert checks['parentBytecodeDisabled'] is True
    assert checks['parentArgv'][1:4]==['-B','-X','pycache_prefix='+checks['parentCachePrefix']]
    if 'uv-' in name or name=='core-comparison-uv-alias':
        launch=read(stage/'launch.json'); probe=read(stage/'uv-identity.json')
        assert launch['requestedChildArgv']==checks['requestedChildArgv'] and inp['argv'][7:]==launch['requestedChildArgv']
        assert probe['exitCode']==0
        for stream in ('stdout','stderr'):assert sha(stage/('uv-identity.'+stream))==probe[stream+'Sha256']
        identity=read(stage/'uv-identity.stdout')
        assert identity=={'executable':'/home/charl/Moriarty/.venv/bin/python3','prefix':'/home/charl/Moriarty/.venv',
            'resolved':'/home/charl/.local/share/uv/python/cpython-3.13.14-linux-x86_64-gnu/bin/python3.13','version':[3,13,14]}
        env=launch['environment']
        assert env['UV_NO_SYNC']==env['UV_OFFLINE']==env['UV_NO_CONFIG']==env['UV_NO_ENV_FILE']=='1'
        assert env['UV_PYTHON_DOWNLOADS']=='never' and env['UV_PROJECT_ENVIRONMENT']==identity['prefix']
        assert env['PATH'].split(':')[0]==str(R/'task2-uv-runtime/bin')
    requested=checks.get('requestedChildArgv',inp['argv'])
    if name!='core-comparison-uv-alias':
        assert requested[:6]==['/home/charl/Moriarty/.venv/bin/python','-m','pytest','-q','-c','pyproject.toml']
        assert requested[7:]==list(fixture['originalTests'])
        assert requested[6]=='--collect-only' if name.endswith('collect') else requested[6]=='--junitxml='+str(stage/'junit.xml')
assert '440 passed' in (R/'task2-original-python-regression/stdout.txt').read_text()
assert 'FileNotFoundError' in (R/'task2-original-python-regression/stdout.txt').read_text()
assert 'AssertionError' in (R/'task2-original-python-uv-collect/stderr.txt').read_text()
assert read(R/'core-comparison-uv-alias/report.json')=={'ok':True,'differences':[],'scope':'complete-inventory'}
final=ROOT/'.superpowers/sdd/candidate-a-export-stages/final'
assert len(read(final/'cases.json')['cases'])==53 and len(list((final/'inputs').glob('*.itf.json')))==14
core=read(R/'core-comparison-uv-alias/fixture-checks.json')['requestedChildArgv']
assert core==['/home/charl/Moriarty/.venv/bin/python','scripts/check_s02_candidate_a_correspondence.py',
    '--cases',str(final/'cases.json'),'--input-root',str(final/'inputs'),'--report',str(R/'core-comparison-uv-alias/report.json')]
alias=Path('/home/charl/Moriarty/.venv/bin/python3')
assert alias.is_symlink() and str(alias.readlink())=='python'
assert str(alias.resolve())=='/home/charl/.local/share/uv/python/cpython-3.13.14-linux-x86_64-gnu/bin/python3.13'
assert members==3650
print(json.dumps({'ok':True,'scope':'Task2-original-evidence-intake-only-not-pilot-or-full-A5',
    'terminalCommands':52,'successfulCommands':49,'preservedExpectedFailures':sorted(failures),
    'originalStageFiles':originals_checked,'archivedSourceMembers':members,'frozenFiles':30,
    'corpusTests':counts,'equivalenceTests':eq_names,'witnessCounts':witness_counts,
    'originalPythonCases':441,'fixtureArchiveMembers':2691,'coreCases':53,'originalCoreItfs':14,
    'stages':stages},indent=2))
