"""Recording fixtures only: no actual Node/Quint/model/QA/native execution."""
from pathlib import Path
import hashlib,json,os,re,shutil,subprocess

root=Path(__file__).resolve().parents[1]
basis=json.loads((root/'BASIS.json').read_text())
old=(root/'preimage/scripts/doctorwatson.sh').read_text()
new=(root/'afterimage/scripts/doctorwatson.sh').read_text()
early='# Explicit single-operation execution:'
assert old[old.index(early):old.index('design=false')]==new[new.index(early):new.index('design=false')]
tail='python3 models/sherlock/qa/check.py --requirement-evidence'
assert old[old.index(tail):]==new[new.index(tail):], 'existing QA/host/design commands must stay byte-for-byte'
match=basis['match_argument']
model=Path(basis['model_source']).read_bytes()
assert hashlib.sha256(model).hexdigest()==basis['model_sha256']
assert re.findall(r'^\s*run\s+(\w+Test)\s*=',model.decode(),re.M)==basis['named_tests']
recorder=r'''#!/usr/bin/python3
import hashlib,json,os,pathlib,sys
log=pathlib.Path(os.environ['TMPDIR'])
args=sys.argv[1:]
entry={'argv':args,'cwd':os.getcwd(),'environment':dict(os.environ)}
if len(args)>3:
 path=pathlib.Path(args[3])
 entry['model_sha256']=hashlib.sha256(path.read_bytes()).hexdigest() if path.is_file() else None
with (log/'node.jsonl').open('a') as stream:stream.write(json.dumps(entry)+'\n')
if args[2]=='typecheck' and (log/'fail-first').exists():sys.exit(17)
if args[2]=='test' and (log/'fail-second').exists():sys.exit(19)
'''
timeout_recorder=r'''#!/usr/bin/python3
import json,os,pathlib,sys
with (pathlib.Path(os.environ['TMPDIR'])/'timeout.jsonl').open('a') as stream:stream.write(json.dumps({'argv':sys.argv[1:],'cwd':os.getcwd()})+'\n')
os.execv('/usr/bin/timeout',['/usr/bin/timeout',*sys.argv[1:]])
'''
poison=r'''#!/usr/bin/python3
import json,os,pathlib,sys
with (pathlib.Path(os.environ['TMPDIR'])/'forbidden.jsonl').open('a') as stream:stream.write(json.dumps({'executable':sys.argv[0],'argv':sys.argv[1:]})+'\n')
sys.exit(99)
'''

def executable(path,text):
    path.write_text(text);path.chmod(0o755)

def records(path):
    return [json.loads(line) for line in path.read_text().splitlines()] if path.exists() else []

def run(name,argv,expected,*,preimage=False,fail=None,defaults=False,missing=False):
    case=root/'receipts/fixture-runs'/name
    if case.exists():shutil.rmtree(case)  # Only this harness's isolated scratch case.
    script=case/'repo/scripts/doctorwatson.sh'
    script.parent.mkdir(parents=True,exist_ok=True)
    script.write_text(old if preimage else new);script.chmod(0o755)
    cwd=case/'repo/models/doctorwatson'
    cwd.mkdir(parents=True,exist_ok=True)
    (cwd/'doctorwatson_delivery_flow.qnt').write_bytes(model)
    caller=case/'caller';caller.mkdir()
    tools=case/'tools';tools.mkdir();logs=case/'logs';logs.mkdir()
    node=tools/('node' if defaults else 'recording node')
    quint=tools/('quint' if defaults else 'recording quint.js')
    executable(node,recorder)
    executable(quint,'// Recording fixture only; not a real Quint invocation.\n')
    executable(tools/'timeout',timeout_recorder)
    for command in ('python3','cargo','rustc','npm','npx','midnight','moriarty'):
        executable(tools/command,poison)
    env=dict(os.environ,PATH=str(tools)+':/usr/bin:/bin',TMPDIR=str(logs),SENTINEL_MUST_NOT_LEAK='private-environment-marker')
    if defaults:
        env.pop('DOCTORWATSON_NODE',None);env.pop('DOCTORWATSON_QUINT',None)
    else:
        env.update(DOCTORWATSON_NODE=str(node),DOCTORWATSON_QUINT=str(quint))
    if missing:env['DOCTORWATSON_NODE']=str(tools/'not-installed')
    if fail:(logs/fail).write_text('fixture fail-stop selector')
    process=subprocess.run(['/usr/bin/bash',str(script),*argv],cwd=caller,env=env,capture_output=True,text=True,timeout=10)
    (case/'stdout').write_text(process.stdout);(case/'stderr').write_text(process.stderr)
    assert process.returncode==expected,(name,process.returncode,process.stderr)
    nodes=records(logs/'node.jsonl');timeouts=records(logs/'timeout.jsonl');forbidden=records(logs/'forbidden.jsonl')
    assert not forbidden,(name,forbidden)
    wanted=0 if expected in (2,127) else 1 if fail=='fail-first' else 2
    assert len(nodes)==len(timeouts)==wanted,(name,nodes,timeouts)
    expected_commands=[
        ['--max-old-space-size=256',str(quint),'typecheck','doctorwatson_delivery_flow.qnt'],
        ['--max-old-space-size=256',str(quint),'test','doctorwatson_delivery_flow.qnt','--main=doctorwatson_delivery_flow','--backend=typescript','--max-samples=1','--seed=1',match]]
    for index,entry in enumerate(nodes):
        assert entry['argv']==expected_commands[index],(name,entry)
        assert entry['cwd']==str(cwd),entry
        assert entry['model_sha256']==basis['model_sha256']
        assert entry['environment']['PATH']=='/usr/bin:/bin'
        assert entry['environment']['LANG']=='C.UTF-8'
        assert entry['environment']['TMPDIR']==str(logs)
        assert 'SENTINEL_MUST_NOT_LEAK' not in entry['environment']
        assert 'DOCTORWATSON_NODE' not in entry['environment'] and 'DOCTORWATSON_QUINT' not in entry['environment']
        expected_timeout=['--signal=KILL','60s','env','-i','PATH=/usr/bin:/bin','LANG=C.UTF-8','TMPDIR='+str(logs),str(node),*expected_commands[index]]
        assert timeouts[index]['argv']==expected_timeout,(name,timeouts[index])
        assert timeouts[index]['cwd']==str(cwd)
    return dict(name=name,exit_code=process.returncode,node_calls=len(nodes),timeout_calls=len(timeouts),forbidden_calls=len(forbidden),expected_result=True)

results=[]
results.append(run('preimage-no-route',['--design-flow'],2,preimage=True))
results.append(run('positive-explicit-tools',['--design-flow'],0))
results.append(run('positive-default-tool-resolution',['--design-flow'],0,defaults=True))
results.append(run('first-failure-stops',['--design-flow'],17,fail='fail-first'))
results.append(run('second-failure-propagates',['--design-flow'],19,fail='fail-second'))
results.append(run('missing-installed-node',['--design-flow'],127,missing=True))
for name,args in [('extra-argument',['--design-flow','extra']),('extra-model',['--design-flow','--model','arbitrary.qnt']),('equal-suffix',['--design-flow=arbitrary']),('unknown',['--unknown-flow'])]:
    results.append(run(name,args,2))
syntax=subprocess.run(['/usr/bin/bash','-n',str(root/'afterimage/scripts/doctorwatson.sh')],capture_output=True,text=True,timeout=10)
assert syntax.returncode==0,syntax.stderr
report=dict(schema='doctorwatson-design-flow-recording-checks/1',scope='recording Node/Quint fixtures only; actual model tests not rerun; no QA/SDK/native execution',canonical_commit=basis['canonical_commit'],script_sha256=hashlib.sha256(new.encode()).hexdigest(),model_dependency_sha256=basis['model_sha256'],named_test_count=len(basis['named_tests']),cases=results,bash_n_exit=syntax.returncode,unchanged_early_dispatch_bytes=True,unchanged_qa_host_old_design_tail_bytes=True,all_pass=True)
(root/'receipts/RESULT.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
