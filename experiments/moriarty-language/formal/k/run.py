#!/usr/bin/env python3
"""Bounded local K commands. Root supplies whole-tree 4 GiB systemd containment."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import time
import codec

HERE=Path(__file__).resolve().parent
BUILD=HERE/'.build'
DEFINITION=BUILD/'moriarty-kompiled'
BINDING=BUILD/'binding.json'
START=time.monotonic()
DEADLINE=START+512
SUITE='initial'
SUITES={'initial':'fixtures/cases.json', 'branches':'fixtures/branches.json', 'transfer-only':'fixtures/transfer-only.json', 'numeric':'fixtures/numeric.json'}

SUITE_LIMITS={'initial':(16,512),'branches':(16,512),'transfer-only':(16,512),'numeric':(64,1512)}

class RunnerError(RuntimeError):pass
def fail(code):raise RunnerError(code)
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def write(path,value):path.write_text(json.dumps(value,indent=2)+'\n')
def sources():return {name:sha(HERE/name) for name in ['moriarty.k','codec.py','run.py','toolchain.lock.json',SUITES[SUITE]]}
def toolchain():
    lock=json.loads((HERE/'toolchain.lock.json').read_text())
    for name in ['kompile','krun']:
        tool=lock['executables'][name];path=Path(tool['path'])
        if not path.is_file() or sha(path)!=tool['sha256']:fail('TOOLCHAIN_STALE')
    return lock

def artifacts():
    if not DEFINITION.is_dir():fail('COMPILED_MISSING')
    result={str(p.relative_to(DEFINITION)):sha(p) for p in sorted(DEFINITION.rglob('*')) if p.is_file()}
    if not result:fail('COMPILED_MISSING')
    return result

def command(argv,seconds,label):
    remaining=DEADLINE-time.monotonic()
    if remaining<=0:fail('AGGREGATE_DEADLINE')
    stdout=BUILD/(label+'.stdout');stderr=BUILD/(label+'.stderr')
    begun=time.monotonic();timed_out=False
    with stdout.open('wb') as out,stderr.open('wb') as err:
        proc=subprocess.Popen(argv,cwd=HERE,stdout=out,stderr=err,start_new_session=True)
        try:code=proc.wait(timeout=min(seconds,remaining))
        except subprocess.TimeoutExpired:
            timed_out=True;os.killpg(proc.pid,signal.SIGKILL);code=proc.wait()
    write(BUILD/(label+'.command.json'),{'argv':argv,'returncode':code,'timedOut':timed_out,'elapsedSeconds':time.monotonic()-begun,'aggregateSeconds':time.monotonic()-START})
    if timed_out:fail('COMMAND_TIMEOUT')
    if code!=0:fail('COMMAND_FAILED')
    if time.monotonic()>=DEADLINE:fail('AGGREGATE_DEADLINE')
    if stdout.stat().st_size>1024*1024:fail('K_OUTPUT_BOUND')
    return stdout.read_text(encoding='utf8')

def compile_definition():
    lock=toolchain();BUILD.mkdir(exist_ok=True)
    before=sources()
    try:
        with (BUILD/'compile-attempt.json').open('x') as f:json.dump({'sources':sources(),'started':time.time()},f)
    except FileExistsError:fail('COMPILE_ALREADY_ATTEMPTED')
    command([lock['executables']['kompile']['path'],str(HERE/'moriarty.k'),'--main-module','MORIARTY','--syntax-module','MORIARTY-SYNTAX','--backend','llvm','--output-definition',str(DEFINITION)],180,'compile')
    if sources()!=before:fail('SOURCE_CHANGED_DURING_COMPILE')
    write(BINDING,{'sources':before,'artifacts':artifacts(),'krunInvocations':0})

def evaluate(packet,label):
    lock=toolchain()
    if not BINDING.is_file():fail('COMPILED_MISSING')
    binding=json.loads(BINDING.read_text())
    if binding['sources']!=sources() or binding['artifacts']!=artifacts():fail('COMPILED_STALE')
    count=binding['krunInvocations']
    if type(count) is not int or count<0 or count>=SUITE_LIMITS[SUITE][0]:fail('KRUN_LIMIT')
    # Count the actual attempt before subprocess dispatch; failures do not refund it.
    binding['krunInvocations']=count+1;write(BINDING,binding)
    name=f'trace-{count+1:02d}'
    program=BUILD/(name+'.input');program.write_text(codec.encode(packet)+'\n')
    raw=command([lock['executables']['krun']['path'],str(program),'--definition',str(DEFINITION),'--output','json'],20,name)
    result=codec.decode(raw,packet)
    write(BUILD/(name+'.result.json'),{'id':label,'inputDigest':codec.digest(packet),'result':result})
    return result

def traces():
    cases=json.loads((HERE/SUITES[SUITE]).read_text())
    if type(cases) is not list or not cases or len(cases)>SUITE_LIMITS[SUITE][0]:fail('FIXTURE_COUNT')
    ids=[c['id'] for c in cases]
    if len(set(ids))!=len(ids):fail('FIXTURE_IDS')
    observations=[]
    for c in cases:
        packet=codec.admit(json.dumps(c['input']))
        result=evaluate(packet,c['id'])
        observations.append({'id':c['id'],'result':result,'matchesExpected':result==c['expected']})
        write(BUILD/'observations.json',observations)
        if result!=c['expected']:fail('EXPECTED_RESULT_MISMATCH')
    return observations

def main():
    global SUITE, DEADLINE
    parser=argparse.ArgumentParser()
    parser.add_argument('--suite',choices=tuple(SUITES),default='initial')
    sub=parser.add_subparsers(dest='command',required=True)
    sub.add_parser('compile')
    sub.add_parser('prove')
    sub.add_parser('evaluate').add_argument('input',type=Path)
    for name in ['traces','compile-and-traces']:
        sub.add_parser(name).add_argument('--all',action='store_true',required=True)
    args=parser.parse_args()
    SUITE=args.suite
    DEADLINE=START+SUITE_LIMITS[SUITE][1]
    if args.command=='prove':fail('PROOF_UNIMPLEMENTED')
    if args.command in ['compile','compile-and-traces']:compile_definition()
    if args.command in ['traces','compile-and-traces']:result=traces()
    elif args.command=='evaluate':
        with args.input.open('rb') as f:raw=f.read(codec.LIMIT+1)
        if len(raw)>codec.LIMIT:fail('INPUT_BOUND')
        result=evaluate(codec.admit(raw.decode('utf8')),'input')
    else:result={'status':'Compiled','binding':str(BINDING)}
    print(json.dumps(result))
    if type(result) is dict and result.get('status')=='Rejected':return 1
    return 0

if __name__=='__main__':
    try:sys.exit(main())
    except (RunnerError,codec.CodecError,OSError,ValueError,KeyError) as exc:
        print(json.dumps({'status':'HarnessError','code':str(exc)}));sys.exit(2)
