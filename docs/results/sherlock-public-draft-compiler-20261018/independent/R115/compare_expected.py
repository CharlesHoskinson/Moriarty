"""Exact R112 selected interface comparison; no subject imports or execution."""
import argparse
from decimal import Decimal
import hashlib,json
from pathlib import Path
P=Path(__file__).resolve().parent
EXPECTED_SHA256='24cc457da9858ecbb6358f2f146f151151ab59ac4213bc4757f44b88060f121b'
FIELDS={'entry','expressions','occurrences','occurrence_paths','children','functions','parameters','arms','patterns','constants','constant_fields','names'}
def load(path):return json.loads(Path(path).read_text(),parse_int=int,parse_float=Decimal)
def golden():
    path=P/'EXPECTED.json'
    if hashlib.sha256(path.read_bytes()).hexdigest()!=EXPECTED_SHA256:raise ValueError('Independent expectation pin mismatch')
    data=load(path)
    if set(data)!={'chain_total','branch_total'}:raise ValueError('Both expected programs required')
    for record in data.values():
        if set(record)!={'source_digest','source','preparation','program_view'} or set(record['program_view'])!=FIELDS:raise ValueError('Complete R112 expectation inventory required')
    return data

def equal(expected,actual,path,errors):
    if type(expected) is not type(actual):errors.append({'path':path,'reason':'type'});return
    if isinstance(expected,dict):
        if set(expected)!=set(actual):errors.append({'path':path,'reason':'keys','missing':sorted(set(expected)-set(actual)),'extra':sorted(set(actual)-set(expected))})
        for key in expected:
            if key in actual:equal(expected[key],actual[key],path+'/'+key,errors)
    elif isinstance(expected,list):
        if len(expected)!=len(actual):errors.append({'path':path,'reason':'cardinality'})
        for index,(left,right) in enumerate(zip(expected,actual)):equal(left,right,path+'/'+str(index),errors)
    elif expected!=actual:errors.append({'path':path,'reason':'value','expected':expected,'actual':actual})

def compare(actual,case):
    expected=golden()[case];errors=[]
    if type(actual) is not dict:raise ValueError('Artifact must be JSON object')
    source=actual.get('source');admission=actual.get('admission')
    record={'source_digest':source.get('source_digest') if type(source) is dict else None,'source':source,'preparation':actual.get('preparation'),'program_view':admission.get('program_view') if type(admission) is dict else None}
    equal(expected,record,'',errors)
    return {'status':'MATCH_COMPLETE_R112_SELECTED_INTERFACE' if not errors else 'MISMATCH_R112_SELECTED_INTERFACE','case':case,'errors':errors,'complete_selected_interface':not errors,'full_artifact_runtime_or_native_acceptance':False,'scope':'Exact source digest, complete source/preparation objects and all12 ProgramView fields only; other artifact/admission/genesis/config/target/runtime fields excluded by R112 interface'}

def main():
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('artifact');parser.add_argument('--case',required=True,choices=['chain-total','branch-total']);args=parser.parse_args()
    try:
        report=compare(load(args.artifact),args.case.replace('-','_'));report['artifact_sha256']=hashlib.sha256(Path(args.artifact).read_bytes()).hexdigest()
        print(json.dumps(report,indent=2,default=str));return 1 if report['errors'] else 0
    except (OSError,ValueError,KeyError,TypeError) as error:print(json.dumps({'status':'INPUT_ERROR','detail':str(error)}));return 2
if __name__=='__main__':raise SystemExit(main())
