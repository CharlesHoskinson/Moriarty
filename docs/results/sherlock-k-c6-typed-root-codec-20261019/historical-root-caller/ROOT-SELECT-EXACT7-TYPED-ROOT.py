"""Root-only fixed exact7 typed-root runtime QA selection using existing F03.launch.

Ordinary adaptation of the retained new8 caller (27149B c3b7a088...). No containment,
launch policy, runner or subject rule is added; the old schema-compare phase is removed
because this profile has one independent whole typed expected. Root must read the full
actual review and this caller, then select explicitly. The author has not imported or
executed this caller or any subject child.
"""
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import resource
import sys

M=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-typed-root-outcome-codec-20261019')
Q=M/'runtime-qa-postcompile-01'
A=Q/'actual'
COMPILED=M/'actual/c6-typed-root-kompiled'
BASE=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-logical-constructor-origin-counterpart-20261019')
PLAN_PIN={'path':'/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-typed-root-outcome-codec-20261019/runtime-qa-postcompile-01/revision02-corrected-source-01/PROPOSED-EXACT7-RUNTIME-OPERATIONS.json','bytes':28398,'sha256':'97b8f40d1cc4f36631f0b0eae0eb218e9f74058866fd5374de8d837e87cec24f'}
FREEZE_PIN={'path':'/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-typed-root-outcome-codec-20261019/runtime-qa-postcompile-01/revision02-corrected-source-01/SOURCE-FREEZE.json','bytes':6769,'sha256':'5465c44954aba0d0d673a586ecb7948427778d34c4bd35f1afaaaa1892286a37'}
CONDITIONS_PIN={'path':str(Q/'ROOT-CONDITIONS.json'),'bytes':3274,'sha256':'fe85cfab5582a285df223efe56c7cfcb5bdca8aa8aafd9d49469631facea2208'}
BINDING_PIN={'path':str(Q/'OPAQUE-EXPECTED-BINDING.json'),'bytes':985,'sha256':'f9caf0c28bdd535b1155da35edfb37f37f3ce1582148e6e5ade6afa65ec870f2'}
RENDER_PIN={'path':str(Q/'RENDER-MANIFEST.json'),'bytes':571,'sha256':'b3e9a149b0aae60427594b93d3e5501ba44a03c918ba513c5f1128ff68c7c610'}
LABELQ_PIN={'path':str(Q/'context/ACTUAL-LABEL-QUALIFICATION.json'),'bytes':4548,'sha256':'6da21906ac1f72d4cc3ac1dc8535a39076d2401b2c44d8843c62ae3e6aa174e9'}
FROZEN=[PLAN_PIN,FREEZE_PIN,CONDITIONS_PIN,BINDING_PIN,RENDER_PIN,LABELQ_PIN]
PLAN=Path(PLAN_PIN['path']);FREEZE=Path(FREEZE_PIN['path']);CONDITIONS=Path(CONDITIONS_PIN['path']);BINDING=Path(BINDING_PIN['path'])
ORDER=['00-data-unit','01-render','02-kast-json','03-reduce','04-print','05-decode','06-compare']
FSIZE=536870912
TOKEN='SHERLOCK_K_ATTEMPT_TOKEN'
CASES=17;LABELS=67
CARRIER_PASS='PASS_LOSSLESS_TYPED_ROOT_QUERY_CARRIER'
CAPTURE_PASS='PASS_COMPLETE_TYPED_ROOT_ANSWER_CAPTURE'
COMPARE_PASS='PASS_WHOLE_TYPED_C6_ROOT_CODEC'
RESULT_PASS='PASS_SCOPED_C6_TYPED_ROOT_CODEC'
SCOPE06='independent-complete-C6-typed-root-codec-17answers'
OUTPUT_SCHEMA='sherlock-c6-typed-root-positive-codec-wire/1'
PROFILE='logical-C6-typed-root-positive-codec/1'
KAST02_PREFIX=b'LblctObserveAll'


def pin(path):
    path=Path(path);raw=path.read_bytes()
    return {'path':str(path),'bytes':len(raw),'sha256':hashlib.sha256(raw).hexdigest()}


def exact(row):
    if (type(row)is not dict or set(row)!={'path','bytes','sha256'} or type(row['path'])is not str
            or type(row['bytes'])is not int or row['bytes']<0 or type(row['sha256'])is not str
            or len(row['sha256'])!=64 or any(c not in '0123456789abcdef' for c in row['sha256'])):
        raise ValueError('exact typed three-key file pin required')
    return row


def checked(row):
    want=exact({k:row[k] for k in ('path','bytes','sha256')})
    if pin(row['path'])!=want:raise ValueError('pin changed: '+row['path'])
    if 'resolved_path' in row and str(Path(row['path']).resolve())!=row['resolved_path']:
        raise ValueError('resolved tool path changed')
    return want


def unique(pairs):
    out={}
    for k,v in pairs:
        if k in out:raise ValueError('duplicate Root metadata key')
        out[k]=v
    return out


def loads(raw):
    return json.loads(raw,object_pairs_hook=unique,
        parse_constant=lambda x:(_ for _ in ()).throw(ValueError(x)))


def read(path):return loads(Path(path).read_bytes())


def save(path,value):
    with Path(path).open('xb') as stream:
        stream.write((json.dumps(value,sort_keys=True,indent=2,allow_nan=False)+'\n').encode())
        stream.flush();os.fsync(stream.fileno())
    return pin(path)


def sync_file(path):
    with Path(path).open('rb') as stream:os.fsync(stream.fileno())


def sync_dir(path):
    fd=os.open(path,os.O_RDONLY|os.O_DIRECTORY)
    try:os.fsync(fd)
    finally:os.close(fd)


def tree_pins(root):
    entries=sorted(Path(root).rglob('*'))
    if any(p.is_symlink() for p in entries):raise ValueError('symlink in generated file inventory')
    return [pin(p) for p in entries if p.is_file()]


def sorted_rows(rows):return sorted(rows,key=lambda r:r['path'])


def canonical_pin(row,path):
    # Producer metadata only: its exact pin must equal the canonical on-disk file.
    exact(row)
    if str(Path(path).resolve())!=path or row!=pin(path):raise ValueError('producer pin differs from canonical on-disk: '+path)
    return row


def is_int(value,want):return type(value)is int and value==want


def same(a,b):
    # Exact typed JSON equality, as existing c6_data.same.
    if type(a)is not type(b):return False
    if type(a)is dict:return a.keys()==b.keys() and all(same(a[k],b[k]) for k in a)
    if type(a)is list:return len(a)==len(b) and all(same(x,y) for x,y in zip(a,b))
    return a==b


def merge(rows):
    out={}
    for row in rows:
        three=exact({k:row[k] for k in ('path','bytes','sha256')})
        if out.setdefault(three['path'],three)!=three:raise ValueError('conflicting pins for one path: '+three['path'])
    return list(out.values())


def carrier_status(phase,expect):
    # 01 full stdout is exactly one PASS status JSON; product/report pins agree with disk.
    query,report=phase['products']
    status=loads(Path(phase['stdout']).read_bytes())
    if type(status)is not dict or set(status)!={'status','product','report'} or status['status']!=CARRIER_PASS:
        raise ValueError('01 stdout is not exactly one carrier PASS status JSON')
    pins={'product':canonical_pin(status['product'],query),'report':canonical_pin(status['report'],report)}
    invariant=read(report)
    if (type(invariant)is not dict or invariant.get('status')!=CARRIER_PASS or invariant.get('product')!=pins['product']
            or invariant.get('source_input')!=expect['source_input'] or invariant.get('compiled_definition')!=expect['definition']
            or invariant.get('compiled_label_map')!=expect['label_map'] or not is_int(invariant.get('actual_label_count'),LABELS)
            or not is_int(invariant.get('case_count'),CASES) or invariant.get('same_whole_typed_query') is not True
            or not is_int(invariant.get('expected_body_reads'),0) or invariant.get('no_model_or_lane_evaluation_in_Python') is not True):
        raise ValueError('01 INVARIANT-REPORT status/product/input/definition/map/count differs')
    return {'status':CARRIER_PASS,**pins}


def capture_status(phase,operand,cases,answer):
    # 05 full stdout status, report/operand agreement, and whole-wire structure. Root computes no model output.
    product,report=phase['products']
    status=loads(Path(phase['stdout']).read_bytes())
    if type(status)is not dict or set(status)!={'status','product','report'} or status['status']!=CAPTURE_PASS:
        raise ValueError('05 stdout is not exactly one capture PASS status JSON')
    pins={'product':canonical_pin(status['product'],product),'report':canonical_pin(status['report'],report)}
    capture=read(report)
    if (type(capture)is not dict or capture.get('status')!=CAPTURE_PASS or capture.get('product')!=pins['product']
            or capture.get('source_input')!=operand['source_input'] or capture.get('printed')!=operand['printed']
            or not is_int(capture.get('case_count'),CASES) or capture.get('all_input_roots_retained') is not True
            or capture.get('all_actual_encoded_lanes_and_decoded_roots_retained') is not True
            or not is_int(capture.get('expected_body_reads'),0)):
        raise ValueError('05 CAPTURE-REPORT status/product/operands/count differs')
    wire=read(product);sort,sorts=answer
    if (type(wire)is not dict or set(wire)!={'schema','profile','answers'} or wire['schema']!=OUTPUT_SCHEMA
            or wire['profile']!=PROFILE or type(wire['answers'])is not list or len(wire['answers'])!=len(cases)):
        raise ValueError('05 whole actual wire schema/profile/count differs')
    for row,case in zip(wire['answers'],cases):
        # Structure only: identity, original root, and every typed lane/decoded argument retained.
        if (type(row)is not dict or set(row)!={'sort','constructor','arguments'} or row['sort']!=sort
                or row['constructor']!='ctAnswer' or type(row['arguments'])is not list or len(row['arguments'])!=len(sorts)
                or any(type(x)is not dict or x.get('sort')!=s for x,s in zip(row['arguments'],sorts))
                or not same(row['arguments'][0],{'sort':'String','value':case['case_id']})
                or not same(row['arguments'][1],case['root'])):
            raise ValueError('05 answer identity/order/original root/typed arity differs')
    return {'status':CAPTURE_PASS,**pins}


def main():
    sys.dont_write_bytecode=True  # F03 import must not create __pycache__ beside reviewed source.
    for row in FROZEN:checked(row)
    ack_path=Path(__file__).resolve().parent/'ROOT-ACTUAL-REVIEW-SELECTION.json'
    ack=read(ack_path)
    if type(ack)is not dict or ack.get('Root_fullread_and_selected') is not True or ack.get('phase_order')!=ORDER:
        raise ValueError('Root full review/caller selection not recorded')
    opinion=checked(ack['actual_final_opinion'])
    if opinion['bytes']==0:raise ValueError('actual final source/exact7 opinion empty')
    current={'selected_caller':pin(Path(__file__).resolve()),'plan':pin(PLAN),'source_freeze':pin(FREEZE),
        'root_conditions':pin(CONDITIONS),'opaque_binding':pin(BINDING)}
    if any(ack.get(k)!=v for k,v in current.items()):
        raise ValueError('Root ACK does not bind current caller/plan/freeze/conditions/opaque index')
    plan=read(PLAN);phases=plan['phases'];dyn=plan['dynamic_metadata']
    if ([x['id'] for x in phases]!=ORDER or plan['no_compile_or_old8_prefix_replay'] is not True
            or plan['callback'].get('oldmain_ledger_retry_unselected') is not True or plan['source_freeze']!=FREEZE_PIN
            or plan['conditions']!=CONDITIONS_PIN or plan['opaque_binding']!=BINDING_PIN
            or plan['actual_label_qualification']!=LABELQ_PIN):
        raise ValueError('frozen exact7 order/operands differ')
    limits={'wall_seconds':30,'sweep_seconds':3,'rlimit_core':0,'rlimit_fsize':FSIZE,'umask':'0o077'}
    if any(plan['limits'].get(k)!=v for k,v in limits.items()) or plan['limits'].get('no_aggregate_CPU_RSS_disk_cap') is not True:
        raise ValueError('plan limits differ')
    for phase in phases:
        # Plan values are used verbatim; this only proves they match the existing F03.launch layout.
        cwd=phase['cwd'];env=phase['environment']
        if (Path(cwd).parent!=A or Path(cwd).name!=phase['id'] or any(phase['limits'].get(k)!=v for k,v in limits.items())
                or phase['inherit_environment'] is not False or phase['stdin']!='DEVNULL'
                or phase['stdout']!=cwd+'/stdout' or phase['stderr']!=cwd+'/stderr'
                or env.get('HOME')!=cwd+'/home' or env.get('TMPDIR')!=cwd+'/tmp'
                or type(env.get(TOKEN))is not str or not env[TOKEN] or type(phase['products'])is not list):
            raise ValueError('phase metadata differs from F03.launch layout: '+phase['id'])
    if '-Xmx512m' not in phases[2]['argv']:raise ValueError('02 JVM heap qualification differs')
    decode_path=Path(dyn['before05']['path']);barrier_path=Path(dyn['before06']['barrier']);manifest_path=Path(dyn['before06']['manifest'])
    if (dyn['before05']['exactkeys']!=['source_input','printed'] or dyn['before06']['exactkeys']!=['actual','expected','durable_freeze']
            or {decode_path.parent,barrier_path.parent,manifest_path.parent}!={A}
            or phases[5]['argv'][-3]!=str(decode_path) or phases[6]['argv'][-1]!=str(manifest_path)):
        raise ValueError('dynamic decode/barrier/manifest operands differ')
    freeze=read(FREEZE);public=exact(freeze['public_input'])
    binding=read(BINDING);expected=exact(binding['expected']);private=exact(binding['private_freeze'])
    if dyn['before05']['source_input']!=public or dyn['before06']['expected']!=expected or (expected['bytes'],private['bytes'])!=(613495,5190):
        raise ValueError('opaque expected/private freeze or source input binding differs')
    for opaque in (expected,private):
        path=Path(opaque['path'])
        if str(path.resolve())!=str(path) or any(p.is_symlink() for p in [path,*path.parents]):
            raise ValueError('opaque operand not canonical symlink-free')
    compiled=plan['compiled15_reuse']
    definition=[r for r in compiled if r['path']==str(COMPILED/'definition.kore')]
    label_map=[r for r in freeze['members'] if r['path']==str(Q/'context/COMPILED-KLABEL-MAP.json')]
    render=read(RENDER_PIN['path']);labelq=read(LABELQ_PIN['path'])
    if (len(compiled)!=15 or len(definition)!=1 or len(label_map)!=1
            or render!={'source_input':public,'compiled_definition':definition[0]}
            or labelq.get('status')!='PASS_ALL67_POSTCOMPILE_LABELS_STATIC_NO_RUNTIME' or labelq.get('map')!=label_map[0]):
        raise ValueError('compiled15/definition/actual67 map binding differs')
    expect01={'source_input':public,'definition':definition[0],'label_map':label_map[0]}
    tools=read(checked(plan['tools'])['path']);closure=read(checked(plan['K_builtin_closure'])['path'])
    callback=[exact(plan['callback'][k]) for k in ('fullsource','full_callable_closure','full_import_projection')]
    raw=[*FROZEN,*freeze['members'],freeze['model_freeze'],*freeze['model_sources'],public,freeze['registry'],
         freeze['output_root_declaration'],*freeze['reused_helper_sources'],*compiled,*tools['entries'],
         *tools['classpath_jar_membership'],*closure['files'],*callback,plan['tools'],plan['K_builtin_closure'],
         labelq['actual_compile_result'],labelq['compiled15'],labelq['primary']['source'],binding['SAFE_handoff'],
         pin(Path(__file__).resolve()),pin(ack_path),opinion]
    merge(raw);rows=list({r['path']:r for r in raw}.values())
    for row in rows:checked(row)
    cases=read(public['path'])['cases']
    if type(cases)is not list or len(cases)!=CASES or len({c['case_id'] for c in cases})!=CASES:
        raise ValueError('current 17 source identities differ')
    registry=[r for r in read(freeze['registry']['path'])['constructors'] if r['name']=='ctAnswer']
    if len(registry)!=1 or len(registry[0]['argument_sorts'])<3 or registry[0]['argument_sorts'][:2]!=['String','CTRoot']:
        raise ValueError('public ctAnswer declaration differs')
    answer=(registry[0]['sort'],registry[0]['argument_sorts'])
    hard=resource.getrlimit(resource.RLIMIT_FSIZE)[1]
    if hard!=resource.RLIM_INFINITY and hard<FSIZE:raise ValueError('Root hard FSIZE below512MiB')
    # Only original F03 import-time declarations/launch are selected; no old main/ledger/retry.
    spec=importlib.util.spec_from_file_location('root_existing_f03_only',callback[0]['path'])
    f03=importlib.util.module_from_spec(spec);spec.loader.exec_module(f03)
    if (f03.CHILD_SECONDS,f03.SWEEP_SECONDS,f03.FILE_SIZE_LIMIT,f03.TOKEN_NAME)!=(30,3,FSIZE,TOKEN):
        raise ValueError('existing F03 constants differ')
    past=f03.tree(str(M/'actual'))
    produced={}

    def all_pins():
        # Source/test membership is the freeze's explicit data, never a listing of itself.
        for directory,names in ((Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-typed-root-outcome-codec-20261019/runtime-qa-postcompile-01/revision02-corrected-source-01/source'),freeze['source_exact_directory']),(Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-typed-root-outcome-codec-20261019/runtime-qa-postcompile-01/revision02-corrected-source-01/tests'),freeze['test_exact_directory'])):
            entries=sorted(directory.iterdir())
            if any(p.is_symlink() or not p.is_file() for p in entries) or [str(p) for p in entries]!=sorted(names):
                raise ValueError('source/test exact membership differs: '+str(directory))
        if any(os.path.lexists(d/'__pycache__') for d in (Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-typed-root-outcome-codec-20261019/runtime-qa-postcompile-01/revision02-corrected-source-01/source'),Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-typed-root-outcome-codec-20261019/runtime-qa-postcompile-01/revision02-corrected-source-01/tests'),BASE/'source')):
            raise ValueError('__pycache__ present in source/helper directory')
        if sorted_rows(tree_pins(COMPILED))!=sorted_rows(compiled):raise ValueError('compiled exact15 bytes/membership differs')
        if f03.tree(str(M/'actual'))!=past:raise ValueError('past compile files/directories changed')
        jars=Path(tools['classpath_pattern']).parent
        if sorted(str(p) for p in jars.glob('*.jar'))!=sorted(r['path'] for r in tools['classpath_jar_membership']):
            raise ValueError('classpath JAR membership differs')
        # Opaque expected/private freeze are hashed only, never parsed; excluded from semantic read counts.
        return {'pins':[checked(r) for r in [*rows,*produced.values()]],'opaque_hash_only':[checked(expected),checked(private)]}

    all_pins()
    if os.path.lexists(A):raise ValueError('fresh QA actual already exists; no retry/reset')
    A.mkdir(mode=0o700);receipts=A/'root-receipts';receipts.mkdir(mode=0o700)
    for phase in phases:
        cwd=Path(phase['cwd']);cwd.mkdir(mode=0o700);(cwd/'home').mkdir(mode=0o700);(cwd/'tmp').mkdir(mode=0o700)
        (receipts/phase['id']).mkdir(mode=0o700)
    selection=save(A/'ROOT-SELECTION.json',{
        'plan':pin(PLAN),'source_freeze':pin(FREEZE),'caller':pin(Path(__file__).resolve()),'actual_review_selection':pin(ack_path),
        'Root_ACK_bound_current':current,'actual_review':opinion,'phase_order':ORDER,'compiled15':compiled,'Root_hard_fsize':hard,
        'opaque_hash_only':[expected,private],'schema_compare_phase':'REMOVED: one independent whole typed expected; 00 unit, actual KJSON frontend and whole model compare remain',
        'render01_status_pins_gate_before02':True,'kast02_acceptance':'clean exit0, nonempty stdout, EMPTY stderr (conservative STOP); ctObserveAll prefix non-gating',
        'compile_warning_53062B_ec611a63':'UNADJUDICATED','no_compile_or_old8_prefix_replay':True,'expected_body_reads':0,
        'Root_opaque_hashes_excluded':True,'no_aggregate_CPU_RSS_disk_cap':True,'stdlib_unpinned':True,'hash_then_reread_not_atomic':True})
    produced[selection['path']]=selection
    local=[r for r in rows if not r['path'].startswith('/nix/store/')]
    results=[];selected=[];failure=None;compare_class='NOT_REACHED';expected_reads=0

    def freeze06(actual):
        # Every 00-05 file/receipt/scratch, static local operand and past compile file; opaque expected excluded.
        if str(Path(actual['path']).resolve())!=actual['path'] or actual!=pin(phases[5]['products'][0]):
            raise ValueError('05 whole actual not canonical/current')
        past_files=[pin(r['path']) for r in f03.tree(str(M/'actual')) if r['kind']=='file']
        members=merge([*local,*produced.values(),*tree_pins(A),*past_files])
        if actual not in members:raise ValueError('05 actual absent from complete barrier')
        for row in members:checked(row);sync_file(row['path'])
        for directory in sorted((p for p in A.rglob('*') if p.is_dir()),reverse=True):sync_dir(directory)
        for directory in sorted({str(Path(r['path']).parent) for r in members}):sync_dir(directory)
        sync_dir(A);sync_dir(Q);sync_dir(M)
        barrier=save(barrier_path,{'expected_body_reads':0,'expected_body_read_scope':SCOPE06,'members':members,
            'actual':actual,'compiled15':compiled,'directories':{'qa_actual':f03.tree(str(A)),'past_compile':past},
            'Root_opaque_hashes_excluded':True,'fsync':'all member files and containing directories before 06 dispatch'})
        manifest={'actual':actual,'expected':expected,'durable_freeze':barrier}
        saved=save(manifest_path,manifest);sync_dir(A)
        produced[barrier['path']]=barrier;produced[saved['path']]=saved
        if read(manifest_path)!=manifest:raise ValueError('Root exact comparison operand binding differs')

    try:
        for phase in phases:
            label=phase['id'];cwd=Path(phase['cwd']);rd=receipts/label
            if any(r['accepted'] is not True for r in results):raise ValueError('prior phase not accepted')
            if sorted(p.name for p in cwd.iterdir())!=['home','tmp'] or any(list((cwd/n).iterdir()) for n in ('home','tmp')):
                raise ValueError('phase cwd/home/tmp not fresh')
            if label=='02-kast-json' and results[-1].get('producer_status_and_pins',{}).get('status')!=CARRIER_PASS:
                raise ValueError('01 producer status/pins not accepted; STOP before02')
            if label=='05-decode':
                # Created only after 04 success: source input and printed stdout pins; no expected read.
                printed=pin(phases[4]['stdout'])
                if printed not in results[-1]['streams'] or printed['bytes']==0:raise ValueError('04 printed stream changed or empty')
                operand={'source_input':public,'printed':printed}
                dm=save(decode_path,operand);sync_dir(A);produced[dm['path']]=dm
                if read(decode_path)!=operand:raise ValueError('decode operand manifest differs')
            if label=='06-compare':
                capture=results[-1].get('capture_status_and_pins',{})
                if capture.get('status')!=CAPTURE_PASS:raise ValueError('05 capture not accepted; STOP before06')
                freeze06(capture['product'])
            pre=save(rd/'ROOT-PRECHECK.json',{'phase':label,**all_pins()})
            command=save(rd/'COMMAND.json',{'id':label,'argv':phase['argv'],'environment':phase['environment'],
                'cwd':phase['cwd'],'stdin':'DEVNULL','limits':phase['limits']})
            retained=read(command['path'])
            if any(retained[k]!=phase[k] for k in ('argv','environment','cwd')):raise ValueError('COMMAND disagrees with frozen phase')
            readset=save(rd/'ROOT-READSET.json',{'argv_file_operands':[pin(p) for p in phase['argv'] if os.path.isfile(p)],
                'declared_pins':'ROOT-PRECHECK static and produced pins','basis':'conservative declared closure, not access instrumentation',
                'expected_parse_selected':label=='06-compare','Root_opaque_hashes_excluded':True})
            record={'id':label};selected.append(label)
            if label=='06-compare':expected_reads=None  # Dispatch may fail after parse but before classification.
            f03.launch(record,phase['argv'],dict(phase['environment']),phase['cwd'],phase['environment'][TOKEN],FSIZE)
            launch=save(rd/'LAUNCH-RESULT.json',record)
            results.append({'id':label,'launch':launch,'accepted':False,'wall_seconds':record.get('elapsed_seconds')})
            inventory=save(rd/'STAGE-INVENTORY.json',{'tree':f03.tree(str(cwd))})
            allowed={'home','tmp','stdout','stderr',*[Path(p).name for p in phase['products']]}
            unexpected=[p.name for p in cwd.iterdir() if p.name not in allowed]
            streams=[pin(phase['stdout']),pin(phase['stderr'])]
            products=[pin(path) for path in phase['products'] if Path(path).is_file()]
            sp=save(rd/'STREAM-PINS.json',streams);pp=save(rd/'PRODUCT-PINS.json',products)
            results[-1]['streams']=streams
            for row in [pre,command,readset,launch,inventory,sp,pp,*streams,*products,*tree_pins(cwd)]:produced[row['path']]=row
            post=save(rd/'ROOT-POSTCHECK.json',{'phase':label,**all_pins(),'unexpected':unexpected})
            produced[post['path']]=post
            clean=(record.get('outcome')=='exited' and record.get('timed_out') is False
                and record.get('containment_breach') is False and record.get('remaining')==[]
                and not unexpected and len(products)==len(phase['products']) and all(r['bytes']>0 for r in products))
            stdout=Path(phase['stdout']).read_bytes();stderr=Path(phase['stderr']).read_bytes()
            ok=clean and record.get('returncode')==0
            if label in ('02-kast-json','04-print'):ok=ok and bool(stdout)
            if label=='01-render':
                try:check=carrier_status(phase,expect01) if ok else {'not_evaluated':'01 not clean exit0'}
                except Exception as error:check={'failure':type(error).__name__+': '+str(error)}
                ps=save(rd/'PRODUCER-STATUS-PINS.json',check);produced[ps['path']]=ps
                results[-1]['producer_status_and_pins']=check;ok=ok and check.get('status')==CARRIER_PASS
            if label=='02-kast-json':
                # Existing conservative empty-stderr class; actual ctObserveAll prefix is observation only.
                kast=save(rd/'KAST02-ACCEPTANCE.json',{'stdout_nonempty':bool(stdout),'stderr_empty':not stderr,
                    'ctObserveAll_prefix_observed_nongating':stdout.startswith(KAST02_PREFIX),'streams':streams})
                produced[kast['path']]=kast;results[-1]['kast02_acceptance']=kast;ok=ok and not stderr
            if label=='05-decode':
                try:check=capture_status(phase,read(decode_path),cases,answer) if ok else {'not_evaluated':'05 not clean exit0'}
                except Exception as error:check={'failure':type(error).__name__+': '+str(error)}
                cs=save(rd/'CAPTURE-STATUS-PINS.json',check);produced[cs['path']]=cs
                results[-1]['capture_status_and_pins']=check;ok=ok and check.get('status')==CAPTURE_PASS
            if label=='06-compare':
                try:comparison=loads(stdout)
                except Exception:comparison=None
                compare_class='INFRASTRUCTURE_EXCEPTION_OR_INVALID_COMPARISON_STREAM'
                compared=results[-2]['capture_status_and_pins']['product']
                if (clean and not stderr and type(comparison)is dict and comparison.get('actual')==compared
                        and comparison.get('expected')==expected):
                    if record.get('returncode')==0 and comparison.get('status')==COMPARE_PASS and comparison.get('whole_typed_equal') is True:
                        compare_class='PASS_WHOLE_TYPED';expected_reads=1
                    elif record.get('returncode')==1 and comparison.get('status')=='FAIL' and comparison.get('whole_typed_equal') is False:
                        compare_class='MISMATCH_NO_REPAIR_REDERIVE_OR_RETRY';expected_reads=1
                results[-1]['comparison_classification']=compare_class;ok=compare_class=='PASS_WHOLE_TYPED'
            results[-1]['accepted']=ok;sync_dir(rd)
            if not ok:raise ValueError('fail-stop child: '+label)
    except Exception as error:
        failure={'type':type(error).__name__,'message':str(error)}
    status=RESULT_PASS if failure is None and len(results)==7 and all(r['accepted'] for r in results) else (
        'C6_TYPED_ROOT_MISMATCH' if compare_class.startswith('MISMATCH') else 'INFRASTRUCTURE_STOP')
    outcome={'status':status,'phases':results,'failure':failure,'selected_dispatches':selected,
        'unselected':[x for x in ORDER if x not in selected],'comparison_classification':compare_class,
        'expected_body_reads':expected_reads,'expected_parse_dispatch_selected':'06-compare' in selected,
        'expected_parse_attempted_count_unknown':expected_reads is None and '06-compare' in selected,
        'Root_private_expected_semantic_reads':0,'Root_opaque_hashes_excluded':True,'no_compile_or_old8_prefix_replay':True,
        'source_model_inputs_expected_argv_env_or_caps_changed':False,
        'scope':'Positive UNADMITTED proposed C6P1 typed-root codec only; no primitive/publication/admission/StepInv/HostSnapshot/native/Bclock/refinement/auth claim',
        'final_qualifications':{'fixtures':'17 UNADMITTED','JVM_heap_fit':'UNOBSERVED; no headroom claim',
            'compile_warning_53062B_ec611a63':'UNADJUDICATED','stdlib_unpinned':True,'hash_then_reread_not_atomic':True,
            'no_aggregate_CPU_RSS_disk_cap':True},
        'read_count_semantics':'None = parse dispatch attempted, count unknown, never zero; Root opaque hashes are not semantic reads',
        'selection':selection}
    save(A/'ROOT-RESULT.json',outcome);sync_dir(A)
    print(json.dumps(outcome,sort_keys=True))
    if status!=RESULT_PASS:raise SystemExit(1)


if __name__=='__main__':main()
