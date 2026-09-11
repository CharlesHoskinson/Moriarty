#!/usr/bin/env python3
"""Bounded local K commands. Root supplies whole-tree 4 GiB systemd containment."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import signal
import subprocess
import sys
import time
import codec
import expression_codec

HERE=Path(__file__).resolve().parent
BUILD=HERE/'.build'
DEFINITION=BUILD/'moriarty-kompiled'
BINDING=BUILD/'binding.json'
START=time.monotonic()
DEADLINE=START+512
SUITE='initial'
SUITES={'initial':'fixtures/cases.json', 'branches':'fixtures/branches.json', 'transfer-only':'fixtures/transfer-only.json', 'numeric':'fixtures/numeric.json','expression-v1':'fixtures/expression-v1.json'}

SUITE_LIMITS={'initial':(16,512),'branches':(16,512),'transfer-only':(16,512),'numeric':(64,1512),'expression-v1':(115,2552)}
OUTPUT_LIMITS={'initial':1024*1024,'branches':1024*1024,'transfer-only':1024*1024,'numeric':1024*1024,'expression-v1':64*1024*1024}

# These names are the only labels emitted by expression_codec.encode_request.
# The configuration labels are produced by the compiled initializer, before the
# macro phase.  Both sets are checked against the exact alias vocabulary below.
EXPRESSION_INPUT_LABELS=frozenset({'expressionRequest','ejNull','ejString','ejNumber','ejBool','ejArray','ejObject','ejPair','ejNil','ejCons'})
GENERATED_CONFIGURATION_LABELS=frozenset({'<generatedTop>','<k>','<sigma>','<request>','<pre>','<args>','<obs>','<localTypes>','<locals>','<written>','<writes>','<descriptors>','<sourceBytes>','<initialWork>','<work>','<reducing>','<out>','<generatedCounter>'})
EXPECTED_SYNTAX_ALIASES=frozenset({'weakExistsFinally','weakAlwaysFinally','allPathGlobally'})
EMPTY_MACROS_KORE=b'// macros\n'
EMPTY_MACROS_KORE_SHA256='3585c084b81281ec26971b2e42c56b9968a802c5d645a81062296350493491cf'

class RunnerError(RuntimeError):pass
def fail(code):raise RunnerError(code)
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def write(path,value):
    telemetry=SUITE=='expression-v1' and path.name.endswith('.command.json')
    path.write_text((json.dumps(value,indent=2) if telemetry or SUITE!='expression-v1' else expression_codec._canonical(value))+'\n')
def sources():
    names=['moriarty.k','codec.py','run.py','toolchain.lock.json',SUITES[SUITE]]
    if SUITE=='expression-v1':
        names=['run.py','toolchain.lock.json','expression-toolchain.lock.json','expression_codec.py','expression-parser.py',SUITES[SUITE],'expression-json.k','expression-wire.k','expression-types.k','expression-schema.k','expression-values.k','expression-shape.k','expression-infer.k','expression-v1.k']
    return {name:sha(HERE/name) for name in names}
def select_suite(name):
    global SUITE,BUILD,DEFINITION,BINDING,DEADLINE
    if name not in SUITES:fail('SUITE')
    SUITE=name;DEADLINE=START+SUITE_LIMITS[name][1]
    BUILD=HERE/('.build-expression-v1' if name=='expression-v1' else '.build')
    DEFINITION=BUILD/('expression-v1-kompiled' if name=='expression-v1' else 'moriarty-kompiled')
    BINDING=BUILD/'binding.json'

def toolchain():
    lock=json.loads((HERE/'toolchain.lock.json').read_text())
    for name in ['kompile','krun']:
        tool=lock['executables'][name];path=Path(tool['path'])
        if not path.is_file() or sha(path)!=tool['sha256']:fail('TOOLCHAIN_STALE')
    if SUITE=='expression-v1':
        ext=json.loads((HERE/'expression-toolchain.lock.json').read_text())
        if sha(HERE/'toolchain.lock.json')!=ext['baseLockSha256']:fail('TOOLCHAIN_STALE')
        if sha(Path(ext['kast']['path']))!=ext['kast']['sha256']:fail('TOOLCHAIN_STALE')
    return lock

def artifacts():
    if not DEFINITION.is_dir():fail('COMPILED_MISSING')
    result={str(p.relative_to(DEFINITION)):sha(p) for p in sorted(DEFINITION.rglob('*')) if p.is_file()}
    if not result:fail('COMPILED_MISSING')
    return result

def kast_labels(term):
    """Iteratively collect KApply labels from a locally generated KAST term."""
    labels=set();pending=[term]
    while pending:
        node=pending.pop()
        if type(node) is not dict:continue
        if node.get('node')=='KApply':
            label=node.get('label');args=node.get('args')
            if type(label) is not dict or type(label.get('name')) is not str or type(args) is not list:fail('MACRO_GUARD')
            labels.add(label['name']);pending.extend(args)
        elif node.get('node')=='KSequence':
            items=node.get('items')
            if type(items) is not list:fail('MACRO_GUARD')
            pending.extend(items)
    return frozenset(labels)

def expression_krun_argv(argv,term):
    """Append the K bypass only for the reviewed macro-free compiled artifact."""
    macros=DEFINITION/'macros.kore';syntax=DEFINITION/'syntaxDefinition.kore'
    if not macros.is_file() or not syntax.is_file():fail('MACRO_GUARD')
    content=macros.read_bytes()
    if content!=EMPTY_MACROS_KORE or hashlib.sha256(content).hexdigest()!=EMPTY_MACROS_KORE_SHA256:fail('MACRO_GUARD')
    aliases=frozenset(re.findall(r'\balias\s+([A-Za-z][A-Za-z0-9_]*)\s*\{',syntax.read_text(encoding='utf8')))
    if aliases!=EXPECTED_SYNTAX_ALIASES:fail('MACRO_GUARD')
    labels=kast_labels(term)
    if not labels<=EXPRESSION_INPUT_LABELS or labels & aliases or GENERATED_CONFIGURATION_LABELS & aliases:fail('MACRO_GUARD')
    return [*argv,'--no-expand-macros']

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
    if stdout.stat().st_size+stderr.stat().st_size>OUTPUT_LIMITS[SUITE]:fail('K_OUTPUT_BOUND')
    return stdout.read_text(encoding='utf8')

def compile_definition():
    lock=toolchain();BUILD.mkdir(exist_ok=True)
    before=sources()
    try:
        with (BUILD/'compile-attempt.json').open('x') as f:json.dump({'sources':sources(),'started':time.time()},f)
    except FileExistsError:fail('COMPILE_ALREADY_ATTEMPTED')
    definition='expression-v1.k' if SUITE=='expression-v1' else 'moriarty.k'
    main='MORIARTY-EXPRESSION-V1' if SUITE=='expression-v1' else 'MORIARTY'
    syntax=main if SUITE=='expression-v1' else 'MORIARTY-SYNTAX'
    command([lock['executables']['kompile']['path'],str(HERE/definition),'--main-module',main,'--syntax-module',syntax,'--backend','llvm','--output-definition',str(DEFINITION)],180,'compile')
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
    program=BUILD/(name+'.input')
    if SUITE=='expression-v1':
        term=expression_codec.encode_request(packet['schema'],packet['request'])
        program.write_text(expression_codec.serialize_term({'format':'KAST','version':4,'term':term})+'\n')
    else:program.write_text(codec.encode(packet)+'\n')
    argv=[lock['executables']['krun']['path'],str(program),'--definition',str(DEFINITION),'--output','json']
    if SUITE=='expression-v1':
        argv.extend(['--parser',str(HERE/'expression-parser.py')])
        argv=expression_krun_argv(argv,term)
    raw=command(argv,20,name)
    result=expression_codec.decode_result(raw,packet['schema'],packet['request']) if SUITE=='expression-v1' else codec.decode(raw,packet)
    write(BUILD/(name+'.result.json'),{'id':label,'inputDigest':hashlib.sha256(expression_codec._canonical(packet).encode()).hexdigest() if SUITE=='expression-v1' else codec.digest(packet),'result':result})
    return result

def traces():
    raw=(HERE/SUITES[SUITE]).read_text()
    cases=expression_codec._parse(raw,allow_space=True)['cases'] if SUITE=='expression-v1' else json.loads(raw)
    if type(cases) is not list or not cases or len(cases)>SUITE_LIMITS[SUITE][0]:fail('FIXTURE_COUNT')
    ids=[c['id'] for c in cases]
    if len(set(ids))!=len(ids):fail('FIXTURE_IDS')
    observations=[]
    for c in cases:
        packet={'schema':c['schema'],'request':c['request']} if SUITE=='expression-v1' else codec.admit(json.dumps(c['input']))
        result=evaluate(packet,c['id'])
        observations.append({'id':c['id'],'result':result,'matchesExpected':expression_codec._canonical(result)==expression_codec._canonical(c['expected'])})
        write(BUILD/'observations.json',observations)
        if expression_codec._canonical(result)!=expression_codec._canonical(c['expected']):fail('EXPECTED_RESULT_MISMATCH')
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
    if args.suite=='expression-v1':select_suite(args.suite)
    else:
        SUITE=args.suite;DEADLINE=START+SUITE_LIMITS[SUITE][1]
    if args.command=='prove':fail('PROOF_UNIMPLEMENTED')
    if args.command in ['compile','compile-and-traces']:compile_definition()
    if args.command in ['traces','compile-and-traces']:result=traces()
    elif args.command=='evaluate':
        limit=8_000_000 if SUITE=='expression-v1' else codec.LIMIT
        with args.input.open('rb') as f:raw=f.read(limit+1)
        if len(raw)>limit:fail('INPUT_BOUND')
        packet=expression_codec._parse(raw.decode('utf8'),allow_space=True) if SUITE=='expression-v1' else codec.admit(raw.decode('utf8'))
        if SUITE=='expression-v1' and (type(packet) is not dict or set(packet)!={'schema','request'} or any(type(v) is not str for v in packet.values())):fail('INPUT_SCHEMA')
        result=evaluate(packet,'input')
    else:result={'status':'Compiled','binding':str(BINDING)}
    print(expression_codec._canonical(result) if SUITE=='expression-v1' else json.dumps(result))
    if type(result) is dict and result.get('status')=='Rejected':return 1
    return 0

if __name__=='__main__':
    try:sys.exit(main())
    except (RunnerError,codec.CodecError,expression_codec.CodecError,OSError,ValueError,KeyError) as exc:
        print(json.dumps({'status':'HarnessError','code':str(exc)}));sys.exit(2)
