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

def command(argv,seconds,label,output_limit=1024*1024):
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
    if stdout.stat().st_size>output_limit:fail('K_OUTPUT_BOUND')
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

def expression_diagnostic_main():
    """New WSL lineage: one build and one diagnostic, never conformance acceptance.

    Root supplies the reviewed systemd memory, hard stack and wall-time limits.
    Historical attempts are retained in the lock and are not reset by this build.
    """
    global BUILD, DEFINITION, BINDING, DEADLINE
    parser=argparse.ArgumentParser()
    parser.add_argument('--suite',choices=['expression-v1'],required=True)
    sub=parser.add_subparsers(dest='command',required=True)
    sub.add_parser('compile')
    sub.add_parser('compile-and-traces')
    sub.add_parser('traces')
    diagnostic=sub.add_parser('diagnostic')
    diagnostic.add_argument('input',type=Path)
    diagnostic.add_argument('--gdb-command',type=Path,required=True)
    args=parser.parse_args()
    BUILD=HERE/'.build-expression-v1'
    DEFINITION=BUILD/'expression-v1-kompiled'
    BINDING=BUILD/'binding.json'
    DEADLINE=START+3000
    lock=json.loads((HERE/'expression-toolchain.lock.json').read_text())
    for name,digest in lock['files'].items():
        if sha(Path(name))!=digest:fail('TOOLCHAIN_STALE')
    if any(Path(name).exists() for name in lock['absentFiles']):fail('DEBUGGER_STARTUP_STALE')
    if any(any(Path(name).iterdir()) for name in lock['emptyDirectories']):fail('DEBUGGER_STARTUP_STALE')
    names=[p.name for p in sorted(HERE.glob('expression-*.k'))]
    names+=['expression_codec.py','expression-parser.py','expression-toolchain.lock.json','run.py','fixtures/expression-conformance.json']
    before={name:sha(HERE/name) for name in names}
    BUILD.mkdir(exist_ok=True)
    if args.command in ('compile','compile-and-traces'):
        with (BUILD/'compile-attempt.json').open('x') as f:
            json.dump({'sources':before,'started':time.time(),'historicalAttempts':lock['historicalAttempts'],'newCompileAttempt':1},f)
        command([lock['executables']['kompile']['path'],str(HERE/'expression-v1.k'),'--main-module','MORIARTY-EXPRESSION-V1','--syntax-module','MORIARTY-EXPRESSION-V1','--backend','llvm','--output-definition',str(DEFINITION)],180,'compile')
        if before!={name:sha(HERE/name) for name in names}:fail('SOURCE_CHANGED_DURING_COMPILE')
        write(BINDING,{'sources':before,'artifacts':artifacts(),'historicalAttempts':lock['historicalAttempts'],'krunInvocations':0})
        if args.command=='compile':
            print(json.dumps({'status':'Compiled','provenance':'new-wsl-build','binding':str(BINDING)}))
            return 0
    binding=json.loads(BINDING.read_text())
    if binding['sources']!=before or binding['artifacts']!=artifacts():fail('COMPILED_STALE')
    if args.command in ('traces','compile-and-traces'):
        import expression_codec as expression
        cases=expression._parse((HERE/'fixtures/expression-conformance.json').read_text(),allow_space=True)['cases']
        if len(cases)!=125 or len({c['id'] for c in cases})!=125:fail('FIXTURE_COUNT')
        observations=[]
        for case in cases:
            count=binding['krunInvocations']
            if type(count) is not int or not 0<=count<125:fail('KRUN_LIMIT')
            label=f'trace-{count+1:03d}'
            program=BUILD/(label+'.input')
            term=expression.encode_request(case['schema'],case['request'])
            program.write_text(expression.serialize_term({'format':'KAST','version':4,'term':term})+'\n')
            binding['krunInvocations']=count+1;write(BINDING,binding)
            raw=command([lock['executables']['krun']['path'],str(program),'--definition',str(DEFINITION),'--output','json','--parser',str(HERE/'expression-parser.py'),'--no-expand-macros'],20,label,64*1024*1024)
            result=expression.decode_result(raw,case['schema'],case['request'])
            result_text=expression.serialize_term(result)
            matches=result_text==expression.serialize_term(case['expected'])
            (BUILD/(label+'.result.json')).write_text(result_text+'\n')
            observations.append({'id':case['id'],'matchesExpected':matches,'inputSha256':sha(program),'resultSha256':hashlib.sha256(result_text.encode()).hexdigest()})
            write(BUILD/'observations.json',observations)
            print(json.dumps({'case':len(observations),'id':case['id'],'matchesExpected':matches}),flush=True)
            if not matches:fail('EXPECTED_RESULT_MISMATCH')
        if before!={name:sha(HERE/name) for name in names}:fail('SOURCE_CHANGED_DURING_RUN')
        return 0
    if sha(args.input)!='6c29dc49a828956f5056605b672b519a515129383c7dd46b0dc34bba174c3c1b':fail('DIAGNOSTIC_INPUT_STALE')
    if sha(args.gdb_command)!=lock['gdbCommandSha256']:fail('DEBUGGER_COMMAND_STALE')
    with (BUILD/'diagnostic-attempt.json').open('x') as f:
        json.dump({'started':time.time(),'inputSha256':sha(args.input),'gdbSha256':sha(args.gdb_command),'historicalAttempts':lock['historicalAttempts'],'newKrunAttempt':1},f)
    command([lock['executables']['krun']['path'],str(args.input.resolve()),'--definition',str(DEFINITION),'--output','json','--parser',str(HERE/'expression-parser.py'),'--no-expand-macros','--debugger-command',str(args.gdb_command.resolve()),'--debugger-batch'],20,'diagnostic')
    print(json.dumps({'status':'DiagnosticFinished','semanticAcceptance':False,'requiresTranscriptReview':True}))
    return 0

def lifecycle_main():
    """Execute a frozen lifecycle corpus against an already bound native artifact."""
    global BUILD, DEFINITION, DEADLINE
    import lifecycle_codec as lifecycle
    import expression_codec as expression
    parser=argparse.ArgumentParser()
    parser.add_argument('--suite',choices=['lifecycle-v1'],required=True)
    parser.add_argument('command',choices=['traces'])
    parser.add_argument('--fixture',type=Path,required=True)
    parser.add_argument('--attempt',required=True)
    args=parser.parse_args()
    if not args.attempt.isalnum():fail('ATTEMPT_NAME')
    BUILD=HERE/'.build-lifecycle-v1';DEFINITION=BUILD/'lifecycle-v1-kompiled';DEADLINE=START+3000
    lock=json.loads((HERE/'expression-toolchain.lock.json').read_text())
    for name,digest in lock['files'].items():
        if sha(Path(name))!=digest:fail('TOOLCHAIN_STALE')
    binding_path=BUILD/'binding.json';binding=json.loads(binding_path.read_text())
    if binding['sources']!={n:sha(HERE/n) for n in binding['sources']} or binding['artifacts']!=artifacts():fail('COMPILED_STALE')
    fixture_digest=sha(args.fixture);cases=json.loads(args.fixture.read_text())['cases']
    if not 0<len(cases)<=128 or len({c['id'] for c in cases})!=len(cases):fail('FIXTURE_COUNT')
    with (BUILD/(args.attempt+'-attempt.json')).open('x') as f:
        json.dump({'fixtureSha256':fixture_digest,'runnerSha256':sha(HERE/'run.py'),'started':time.time(),'maxInvocations':len(cases),'priorInvocations':binding['krunInvocations']},f)
    observations=[];previous={}
    for case in cases:
        packet=case['packet'].copy();chain=case.get('chain')
        if chain and chain.get('previous'):
            predecessor=previous[chain['previous']]
            request=json.loads(packet['request']);request['Pre']=predecessor['post'];request['workInitial']=predecessor['workRemaining']
            packet['request']=expression._canonical(request);packet['financialPreState']=expression._canonical(predecessor['financialPost'])
        admitted=lifecycle.admit_packet(packet);term=lifecycle.encode_packet(admitted)
        label=f"{args.attempt}-{len(observations)+1:03d}";program=BUILD/(label+'.input')
        program.write_text(expression.serialize_term({'format':'KAST','version':4,'term':term})+'\n')
        binding['krunInvocations']+=1;write(binding_path,binding)
        raw=command([lock['executables']['krun']['path'],str(program),'--definition',str(DEFINITION),'--output','json','--parser',str(HERE/'lifecycle-parser.py'),'--no-expand-macros'],20,label,64*1024*1024)
        result=lifecycle.decode_result(raw,admitted);matches=expression._canonical(result)==expression._canonical(case['expected'])
        write(BUILD/(label+'.result.json'),result)
        observations.append({'id':case['id'],'matchesExpected':matches,'result':result});write(BUILD/(args.attempt+'-observations.json'),observations)
        print(json.dumps({'case':len(observations),'id':case['id'],'matchesExpected':matches}),flush=True)
        if not matches:fail('EXPECTED_RESULT_MISMATCH')
        previous[case['id']]=result
    if sha(args.fixture)!=fixture_digest:fail('FIXTURE_CHANGED')
    if binding['sources']!={n:sha(HERE/n) for n in binding['sources']}:fail('SOURCE_CHANGED_DURING_RUN')
    return 0

if __name__=='__main__':
    try:sys.exit(lifecycle_main() if '--suite' in sys.argv and sys.argv[sys.argv.index('--suite')+1:sys.argv.index('--suite')+2]==['lifecycle-v1'] else expression_diagnostic_main() if '--suite' in sys.argv and sys.argv[sys.argv.index('--suite')+1:sys.argv.index('--suite')+2]==['expression-v1'] else main())
    except (RunnerError,codec.CodecError,OSError,ValueError,KeyError) as exc:
        print(json.dumps({'status':'HarnessError','code':str(exc)}));sys.exit(2)
