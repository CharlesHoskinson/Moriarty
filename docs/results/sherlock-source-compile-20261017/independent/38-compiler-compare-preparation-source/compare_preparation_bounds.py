"""Add frozen preparation/bounds predicates to the sealed R26 comparator."""
import argparse
from copy import deepcopy
import hashlib
import importlib.util
import json
from pathlib import Path
import sys
sys.dont_write_bytecode=True
P=Path(__file__).resolve().parent
BASE=P.parent/'26-compiler-compare-source'
BASE_FREEZE='9d54399549c9e677b6a9700c52968c127eb6b25219222133d5fca5f896f3f712'
BASE_SOURCE='98a78b834f088fe4b77645a26f57823539dcc4d6251ff8bd8ae487c9254dd18f'
BINDINGS='23818db8739e7c0d524519b96bf3cf619ef463cced33098f64dbaf77ee086c11'

def digest(path):return hashlib.sha256(Path(path).read_bytes()).hexdigest()
if digest(BASE/'SOURCE-FREEZE.json')!=BASE_FREEZE or digest(BASE/'compare_compile_only.py')!=BASE_SOURCE:raise ValueError('R26 source/freeze pin mismatch')
for member in json.loads((BASE/'SOURCE-FREEZE.json').read_text())['files']:
    path=BASE/member['path']
    if path.stat().st_size!=member['bytes'] or digest(path)!=member['sha256']:raise ValueError('R26 member mismatch: '+member['path'])
if digest(P/'PUBLIC-BINDINGS.json')!=BINDINGS:raise ValueError('Public body-free binding pin mismatch')
spec=importlib.util.spec_from_file_location('sealed_independent_r26',BASE/'compare_compile_only.py')
base=importlib.util.module_from_spec(spec);spec.loader.exec_module(base)
ORACLE=base.ORACLE
TARGET_FILE=base.TARGET_FILE
load=base.load

def compare(artifact,case,oracle=ORACLE):
    if type(artifact) is not dict:raise ValueError('Artifact must be a JSON object')
    report=base.compare(artifact,case,oracle)
    errors=report['errors'];unselected=[]
    frozen=load(Path(oracle)/('EXPECTED-QUOTE-SOURCE-REPORT-PROJECTION.json' if case=='quote' else 'EXPECTED-ELIGIBILITY-SOURCE-REPORT-PROJECTION.json'))
    expected_bounds=load(Path(oracle)/'EXPECTED-IDENTITY-LAYOUT-GENESIS.json')['layout']['bounds']
    try:actual_bounds=base.pointer(artifact,'/admission/bounds')
    except (KeyError,IndexError,TypeError,ValueError):actual_bounds=None
    base.strict(expected_bounds,actual_bounds,'/admission/bounds',errors)
    def issue(path,reason):errors.append({'path':path,'reason':reason})
    def keys(value,fields,path):
        if type(value) is not dict:issue(path,'expected_object');return False
        if set(value)!=set(fields):issue(path,'public_struct_keys_differ')
        return True
    def integer(value,path,maximum=None):
        if type(value) is not int or value<0 or (maximum is not None and value>maximum):issue(path,'expected_u16_json_integer' if maximum is not None else 'expected_nonnegative_usize_json_integer')
    def string(value,path):
        if type(value) is not str:issue(path,'expected_string')
    def expr_id(value,path,optional=False):
        if optional and value is None:return
        integer(value,path,65535)
    def unsupported(path,reason):unselected.append({'path':path,'status':'UNSELECTED_VALUE','reason':reason})
    def source_expr(value,path):
        unsupported(path,'Exact CoreExpr value/identity is not in frozen independent expectations')
        if value is None:return
        if type(value) is not dict:issue(path,'expected_nullable_CoreExpr_object');return
        kind=value.get('kind')
        fields={'LiabilityPresent':['kind','name'],'LiabilityAmount':['kind','name'],'Literal':['kind','value'],'Name':['kind','name'],'Binary':['kind','op','lhs','rhs'],'Not':['kind','value']}
        if type(kind) is not str or kind not in fields:issue(path+'/kind','expected_public_CoreExpr_variant');return
        keys(value,fields[kind],path)
        if kind in ['Name','LiabilityPresent','LiabilityAmount']:string(value.get('name'),path+'/name')
        elif kind=='Binary':
            if value.get('op') not in ['Add','Sub','Mul','Eq','Ne','Lt','Le','Gt','Ge','And','Or']:issue(path+'/op','expected_public_BinaryOp_variant')
            expr_id(value.get('lhs'),path+'/lhs');expr_id(value.get('rhs'),path+'/rhs')
        elif kind=='Not':expr_id(value.get('value'),path+'/value')
        else:
            core_value=value.get('value')
            if not keys(core_value,['type','value'],path+'/value'):return
            tag=core_value.get('type')
            if tag=='Bool':
                if type(core_value.get('value')) is not bool:issue(path+'/value/value','expected_boolean')
            elif tag=='UInt':
                unsupported(path+'/value/value','CoreUInt serializer payload type is absent from selected frozen declarations; presence only, no invented encoding')
            else:issue(path+'/value/type','expected_public_Value_variant')
    preparation=artifact.get('preparation')
    report_fields=['action_index','action','inputs','lets','emit','occurrences','return_type','generated_function_name','symbol_domain']
    if keys(preparation,report_fields,'/preparation'):
        for field in ['action_index','action','inputs','return_type','generated_function_name']:
            base.strict(frozen[field],preparation.get(field),'/preparation/'+field,errors)
        string(preparation.get('symbol_domain'),'/preparation/symbol_domain')
        unsupported('/preparation/symbol_domain','Exact raw symbol-domain spelling is not frozen')
        lets=preparation.get('lets')
        if type(lets) is not list:issue('/preparation/lets','expected_array')
        else:
            if len(lets)!=len(frozen['lets']):issue('/preparation/lets','array_length')
            for i,(actual,expected) in enumerate(zip(lets,frozen['lets'])):
                path='/preparation/lets/'+str(i)
                if not keys(actual,['statement_index','name','expression'],path):continue
                for field in ['statement_index','name']:base.strict(expected[field],actual.get(field),path+'/'+field,errors)
                expr_id(actual.get('expression'),path+'/expression')
                unsupported(path+'/expression','Exact Core parser ExprId is not frozen')
        emit=preparation.get('emit')
        if keys(emit,['statement_index','name','expression','role'],'/preparation/emit'):
            for field in ['statement_index','name','role']:base.strict(frozen['emit'][field],emit.get(field),'/preparation/emit/'+field,errors)
            expr_id(emit.get('expression'),'/preparation/emit/expression')
            unsupported('/preparation/emit/expression','Exact Core parser ExprId is not frozen')
        occurrences=preparation.get('occurrences')
        if type(occurrences) is not list:issue('/preparation/occurrences','expected_array')
        else:
            if len(occurrences)!=frozen['occurrence_count']:issue('/preparation/occurrences','array_length')
            fields=['action_index','statement_index','expression','body_path','node_kind','source_expression']
            for i,(actual,path_expected) in enumerate(zip(occurrences,frozen['occurrences_body_path_order'])):
                path='/preparation/occurrences/'+str(i)
                if not keys(actual,fields,path):continue
                base.strict(frozen['occurrence_other_fields']['action_index'],actual.get('action_index'),path+'/action_index',errors)
                base.strict(path_expected,actual.get('body_path'),path+'/body_path',errors)
                integer(actual.get('statement_index'),path+'/statement_index')
                unsupported(path+'/statement_index','Only source provenance relation, not exact per-occurrence statement index, is frozen')
                expr_id(actual.get('expression'),path+'/expression',optional=True)
                unsupported(path+'/expression','Exact optional Core parser ExprId is not frozen')
                string(actual.get('node_kind'),path+'/node_kind')
                unsupported(path+'/node_kind','Exact raw/synthetic node_kind spelling is not frozen')
                source_expr(actual.get('source_expression'),path+'/source_expression')
    report['unselected']=[x for x in report['unselected'] if x['role']!='preparation_unselected_fields']
    report['preparation_unselected_values']=unselected
    report['raw_preparation']=deepcopy(preparation)
    report['selected_extension']={'admission_bounds':'All22 exact fields; decimal-string type/value/presence','preparation':'Exact nine-field public inventory; frozen values/order/cardinality plus declared primitive/nested types; unsupported raw values listed by exact path','usize_upper_width':'Only nonnegative numeric JSON shape selected for unvalued usize metadata; host ABI upper width not inferred'}
    report['status']='MISMATCH_SELECTED_FIELDS' if errors else 'MATCH_SELECTED_FIELDS_WITH_EXPLICIT_UNSELECTED_FIELDS'
    report['scope']+=' Additive selected preparation and admission.bounds predicates; no exact unsupported source metadata values or full artifact/runtime/native closure.'
    return report

def main():
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('artifact');parser.add_argument('--case',required=True,choices=['quote','eligibility']);args=parser.parse_args()
    try:
        report=compare(load(args.artifact),args.case);report['artifact_sha256']=digest(args.artifact)
        print(json.dumps(report,indent=2,default=str));return 1 if report['errors'] else 0
    except (OSError,ValueError,TypeError,KeyError,IndexError) as exc:
        print(json.dumps({'status':'INPUT_OR_CONTRACT_ERROR','detail':str(exc)}));return 2
if __name__=='__main__':raise SystemExit(main())
