"""Independent selected compile-only comparison. No compiler/owner imports or calls."""
import argparse
from copy import deepcopy
from decimal import Decimal
import hashlib
import json
from pathlib import Path
import re

ORACLE=Path(__file__).resolve().parent.parent/'R20-compiler-oracle-correction02'
PUBLIC_BINDINGS=Path(__file__).resolve().parent/'PUBLIC-WIRE-BINDINGS.json'
TARGET_FILE=Path(__file__).resolve().parent/'inputs/target.json'
BINDINGS_SHA256='179787e098129cca145656dbe255bbd431b9134af20d218ca8f589a9c1292d09'
TARGET_SHA256='795994c1e64aa1703482fa09a995d5e94b09b2741c7f4885a3a3f6b33c8be9b1'
ORACLE_FREEZE_SHA256='f34e552376880e6379bc349439d6753263deb5e9e5497682211ed24e0f18cf07'
DECIMAL=re.compile(r'0|[1-9][0-9]*')

def load(path):
    def object_pairs(pairs):
        result={}
        for key,value in pairs:
            if key in result:raise ValueError('Duplicate JSON object key: '+key)
            result[key]=value
        return result
    return json.loads(Path(path).read_text(),parse_int=int,parse_float=Decimal,object_pairs_hook=object_pairs)

def digest(path):return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def verify_oracle(oracle):
    if digest(oracle/'SOURCE-FREEZE.json')!=ORACLE_FREEZE_SHA256:raise ValueError('Oracle freeze pin mismatch')
    for member in load(oracle/'SOURCE-FREEZE.json')['files']:
        path=oracle/member['path']
        if path.stat().st_size!=member['bytes'] or digest(path)!=member['sha256']:raise ValueError('Oracle member mismatch: '+member['path'])

def strict(expected,actual,path,errors):
    if type(expected) is not type(actual):
        errors.append({'path':path,'reason':'type','expected_type':type(expected).__name__,'actual_type':type(actual).__name__});return
    if isinstance(expected,dict):
        if set(expected)!=set(actual):errors.append({'path':path,'reason':'object_keys','missing':sorted(set(expected)-set(actual)),'extra':sorted(set(actual)-set(expected))})
        for key in expected:
            if key in actual:strict(expected[key],actual[key],path+'/'+key,errors)
    elif isinstance(expected,list):
        if len(expected)!=len(actual):errors.append({'path':path,'reason':'array_length','expected':len(expected),'actual':len(actual)})
        for i,(left,right) in enumerate(zip(expected,actual)):strict(left,right,path+'/'+str(i),errors)
    elif expected!=actual:errors.append({'path':path,'reason':'value','expected':expected,'actual':actual})

def pointer(value,path):
    if not path.startswith('/'):raise ValueError('Selected wire pointer must start with /')
    for key in path[1:].split('/'):
        key=key.replace('~1','/').replace('~0','~')
        value=value[int(key)] if isinstance(value,list) else value[key]
    return value

def alpha(view,expected,adapter):
    """Only rename NameIds through the sealed four-role bijection."""
    if type(view) is not dict or set(view)!=set(expected):raise ValueError('ProgramView must contain exactly the sealed twelve fields')
    names=view['names'];roles=adapter['FixedEligibility']['fresh_roles'];source=adapter['FixedEligibility']['fixed_source_names']
    if type(names) is not list or len(names)!=8 or any(type(n) is not str for n in names):raise ValueError('Eligibility names must be eight strings')
    if len(set(names))!=8 or names!=sorted(names,key=lambda n:n.encode('utf-8')):raise ValueError('Eligibility names must be distinct and UTF8-sorted')
    if not set(source)<=set(names):raise ValueError('Missing fixed source name')
    mapping={name:name for name in source};records=[];ids=set()
    for role in roles:
        row=view['expressions'][int(role['binding_expr'])]
        raw_id=row['Let']['binder']
        if type(raw_id) is not str or not DECIMAL.fullmatch(raw_id):raise ValueError('Role binder must be canonical decimal NameId')
        index=int(raw_id)
        if index>=len(names) or index in ids or names[index] in source:raise ValueError('Fresh roles must be distinct and disjoint from source names')
        ids.add(index);mapping[names[index]]=role['normalized_name']
        records.append({'role':role['role'],'raw_id':raw_id,'raw_name':names[index],'normalized_id':role['normalized_id'],'normalized_name':role['normalized_name']})
    if set(mapping)!=set(names):raise ValueError('Name mapping must cover all actual names bijectively')
    normalized=deepcopy(view)
    normalized['names']=sorted(mapping.values(),key=lambda n:n.encode('utf-8'))
    if normalized['names']!=expected['names']:raise ValueError('Normalized names differ from sealed names')
    new_ids={name:str(i) for i,name in enumerate(normalized['names'])}
    def rename(raw_id):
        if type(raw_id) is not str or not DECIMAL.fullmatch(raw_id) or int(raw_id)>=len(names):raise ValueError('NameId outside actual names')
        return new_ids[mapping[names[int(raw_id)]]]
    for row in normalized['expressions']:
        if type(row) is not dict or len(row)!=1:raise ValueError('Expression must have one externally tagged variant')
        tag,payload=next(iter(row.items()))
        if tag in ['Let','Var']:payload['binder']=rename(payload['binder'])
        if tag=='Call':payload['function_name']=rename(payload['function_name'])
    for row in normalized['functions']:row['function_name']=rename(row['function_name'])
    # Nonempty parameter/match/constructor-related vectors differ directly from
    # these zero-parameter golden tables; no rewriting is authorized there.
    return normalized,records

def compare(artifact,case,oracle=ORACLE,bindings=None,target_file=None):
    oracle=Path(oracle);verify_oracle(oracle);errors=[];unselected=[];mapping=[]
    if digest(PUBLIC_BINDINGS)!=BINDINGS_SHA256:raise ValueError('Public binding pin mismatch')
    public=load(PUBLIC_BINDINGS)
    if bindings is not None and bindings!=public:raise ValueError('Only the pinned public R28 binding declaration is selected')
    target_file=TARGET_FILE if target_file is None else target_file
    if digest(target_file)!=TARGET_SHA256:raise ValueError('Independent target input pin mismatch')
    label='QUOTE' if case=='quote' else 'ELIGIBILITY'
    expected=load(oracle/('EXPECTED-QUOTE-PROGRAM-VIEW.json' if case=='quote' else 'EXPECTED-ELIGIBILITY-ALPHA-PROGRAM-VIEW.json'))
    envelope=load(oracle/'EXPECTED-COMPILE-ONLY.json');ids=load(oracle/'EXPECTED-IDENTITY-LAYOUT-GENESIS.json');inputs=load(oracle/'INPUTS.json');config=load(oracle/'inputs/execution-config.json')
    def selected(path,value):
        try:actual=pointer(artifact,path)
        except (KeyError,IndexError,TypeError,ValueError):errors.append({'path':path,'reason':'missing_or_invalid_path'});return
        strict(value,actual,path,errors)
    for key in ['format','qualification','execution_profile','input_policy','plans','observation']:selected('/'+key,envelope[key])
    selected('/admission/status','admitted');selected('/admission/representation','HostOnly')
    selected('/admission/identity',ids['identity']);selected('/admission/layout',ids['layout'])
    fixture=inputs['fixtures'][0 if case=='quote' else 1]
    source={'language':'moriarty/1','semantics':'moriarty-core/1','agreement':fixture['agreement'],'action':fixture['action'],'action_index':'0','source_digest':fixture['source_sha256']}
    selected('/source',source)
    expected_config=deepcopy(config);expected_config['file_digest']=digest(oracle/'inputs/execution-config.json')
    selected('/config',expected_config)
    selected('/target',{'descriptor':load(target_file),'file_digest':digest(target_file)})
    # Required unselected report/evidence values remain present; no equality is
    # inferred merely because both sides omit a field.
    for path in ['/preparation','/evidence/source_preparation','/evidence/host_observation']:
        try:pointer(artifact,path)
        except (KeyError,IndexError,TypeError,ValueError):errors.append({'path':path,'reason':'required_field_missing'})
    for key in ['native','target','acceptance']:selected('/evidence/'+key,envelope['evidence'][key])
    raw=None;normalized=None
    try:
        raw=pointer(artifact,'/admission/program_view')
        if case=='eligibility':normalized,mapping=alpha(raw,expected,load(oracle/'EXPECTED-ALPHA-ADAPTER.json'))
        else:normalized=deepcopy(raw)
        strict(expected,normalized,'/admission/program_view',errors)
    except (KeyError,IndexError,TypeError,ValueError) as exc:errors.append({'path':'/admission/program_view','reason':'schema_or_alpha_mapping','detail':str(exc)})
    # Config/target/action-index identities are selected by the pinned public
    # R28 declaration, not inferred from artifacts or compiler implementation.
    for role in ['source_preparation_evidence_value','host_observation_evidence_value','preparation_unselected_fields','extra_wrapper_fields']:
        unselected.append({'role':role,'status':'UNSELECTED','reason':'Not an exact selected predicate in sealed oracle'})
    return {'status':'MISMATCH_SELECTED_FIELDS' if errors else 'MATCH_SELECTED_FIELDS_WITH_EXPLICIT_UNSELECTED_FIELDS','case':case,'comparison_complete':False,'errors':errors,'unselected':unselected,'alpha_mapping':mapping,'raw_program_view':raw,'normalized_program_view':normalized,'scope':'All twelve ProgramView fields exact, with only sealed NameId alpha renaming for Eligibility; selected compile-only envelope/identity/layout/source/config/target identity predicates. No full preparation/wrapper/genesis/runtime/native claim.'}

def main():
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('artifact');parser.add_argument('--case',required=True,choices=['quote','eligibility']);parser.add_argument('--oracle-dir',type=Path,default=ORACLE);parser.add_argument('--wire-bindings',type=Path);parser.add_argument('--target-file',type=Path)
    args=parser.parse_args()
    try:
        if args.wire_bindings and digest(args.wire_bindings)!=BINDINGS_SHA256:raise ValueError('Wire binding file pin mismatch')
        report=compare(load(args.artifact),args.case,args.oracle_dir,load(args.wire_bindings) if args.wire_bindings else None,args.target_file)
        report['artifact_sha256']=digest(args.artifact)
        print(json.dumps(report,indent=2,default=str));return 1 if report['errors'] else 0
    except (OSError,ValueError,TypeError,KeyError,IndexError) as exc:
        print(json.dumps({'status':'INPUT_OR_CONTRACT_ERROR','detail':str(exc)}));return 2
if __name__=='__main__':raise SystemExit(main())
