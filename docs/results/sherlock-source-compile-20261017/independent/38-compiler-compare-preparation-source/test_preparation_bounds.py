"""Synthetic golden-copy/mutation checks for selected R38 predicates only."""
from copy import deepcopy
import importlib.util
import json
from pathlib import Path
import sys
sys.dont_write_bytecode=True
P=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('r38_independent_compare',P/'compare_preparation_bounds.py')
module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
O=module.ORACLE
def base_golden(case):
    envelope=module.load(O/'EXPECTED-COMPILE-ONLY.json');identity=module.load(O/'EXPECTED-IDENTITY-LAYOUT-GENESIS.json');inputs=module.load(O/'INPUTS.json');fixture=inputs['fixtures'][case=='eligibility']
    view=module.load(O/('EXPECTED-QUOTE-PROGRAM-VIEW.json' if case=='quote' else 'EXPECTED-ELIGIBILITY-ALPHA-PROGRAM-VIEW.json'))
    if case=='eligibility':
        fresh={'$alpha.gt.left':'zz_gt_left','$alpha.gt.right':'aa_gt_right','$alpha.and.left':'mm_and_left','$alpha.and.right':'bb_and_right'}
        oldnames=view['names'][:];actualnames=sorted([fresh.get(x,x) for x in oldnames],key=lambda x:x.encode())
        def renamed(raw):return str(actualnames.index(fresh.get(oldnames[int(raw)],oldnames[int(raw)])))
        for row in view['expressions']:
            tag,payload=next(iter(row.items()))
            if tag in ['Let','Var']:payload['binder']=renamed(payload['binder'])
            if tag=='Call':payload['function_name']=renamed(payload['function_name'])
        for row in view['functions']:row['function_name']=renamed(row['function_name'])
        view['names']=actualnames
    artifact={k:deepcopy(envelope[k]) for k in ['format','qualification','execution_profile','input_policy','plans','observation']}
    artifact.update({'source':{'language':'moriarty/1','semantics':'moriarty-core/1','agreement':fixture['agreement'],'action':fixture['action'],'action_index':'0','source_digest':fixture['source_sha256']},'config':{**module.load(O/'inputs/execution-config.json'),'file_digest':module.digest(O/'inputs/execution-config.json')},'target':{'descriptor':module.load(module.TARGET_FILE),'file_digest':module.digest(module.TARGET_FILE)},'preparation':{'synthetic_only':'unselected report values'},'admission':{'status':'admitted','representation':'HostOnly','identity':identity['identity'],'layout':identity['layout'],'program_view':view},'evidence':{'source_preparation':'synthetic unselected','host_observation':'synthetic unselected','native':'unrun','target':'unrun','acceptance':'none'}})
    return artifact

def golden(case):
    artifact=base_golden(case)
    label='QUOTE' if case=='quote' else 'ELIGIBILITY'
    expected=module.load(O/('EXPECTED-'+label+'-SOURCE-REPORT-PROJECTION.json'))
    # Unknown raw metadata uses visibly synthetic valid typed payloads. Its exact
    # values are not asserted to be authentic source/producer values.
    preparation={k:deepcopy(expected[k]) for k in ['action_index','action','inputs','return_type','generated_function_name']}
    preparation['lets']=[{'statement_index':x['statement_index'],'name':x['name'],'expression':0} for x in expected['lets']]
    preparation['emit']={**deepcopy(expected['emit']),'expression':0}
    preparation['occurrences']=[{'action_index':0,'statement_index':0,'expression':None,'body_path':path,'node_kind':'SYNTHETIC_UNSELECTED','source_expression':None} for path in deepcopy(expected['occurrences_body_path_order'])]
    preparation['symbol_domain']='SYNTHETIC_UNSELECTED'
    artifact['preparation']=preparation
    artifact['admission']['bounds']=deepcopy(artifact['admission']['layout']['bounds'])
    return artifact

def set_path(value,path,replacement):
    for key in path[:-1]:value=value[key]
    value[path[-1]]=replacement

checks=[]
for case in ['quote','eligibility']:
    artifact=golden(case);before=deepcopy(artifact);report=module.compare(artifact,case)
    assert not report['errors'] and not report['comparison_complete'] and artifact==before
    assert report['raw_preparation']==artifact['preparation'] and report['preparation_unselected_values']
    checks.append(case+'_selected_preparation_bounds_golden_match_raw_retained')
    for key,value in artifact['admission']['bounds'].items():
        candidate=deepcopy(artifact);candidate['admission']['bounds'][key]=str(int(value)+1)
        assert module.compare(candidate,case)['errors'],(case,key)
        checks.append(case+'_changed_bound_'+key+'_detected')
    mutations=[('report_action_index',['action_index'],1),('report_action',['action'],'changed'),('inputs',['inputs'],[{'index':0,'name':'x','ty':'UInt'}]),('let_statement',['lets',0,'statement_index'],1),('let_name',['lets',0,'name'],'changed'),('let_expr_id_type',['lets',0,'expression'],'0'),('let_expr_id_u16_extent',['lets',0,'expression'],65536),('emit_statement',['emit','statement_index'],9),('emit_name',['emit','name'],'changed'),('emit_role',['emit','role'],'changed'),('emit_expr_id_type',['emit','expression'],'0'),('occurrence_action',['occurrences',0,'action_index'],1),('occurrence_statement_type',['occurrences',0,'statement_index'],'0'),('occurrence_expr_id_type',['occurrences',0,'expression'],'0'),('occurrence_path',['occurrences',0,'body_path'],[1]),('occurrence_node_kind_type',['occurrences',0,'node_kind'],1),('return_type',['return_type'],'Bool' if case=='quote' else 'UInt'),('generated_function',['generated_function_name'],'changed'),('symbol_domain_type',['symbol_domain'],0)]
    for label,path,value in mutations:
        candidate=deepcopy(artifact);set_path(candidate['preparation'],path,value);assert module.compare(candidate,case)['errors'],(case,label)
        checks.append(case+'_'+label+'_detected')
    candidate=deepcopy(artifact);candidate['preparation']['lets'].reverse();assert module.compare(candidate,case)['errors'];checks.append(case+'_let_order_detected')
    candidate=deepcopy(artifact);candidate['preparation']['occurrences'][0],candidate['preparation']['occurrences'][1]=candidate['preparation']['occurrences'][1],candidate['preparation']['occurrences'][0];assert module.compare(candidate,case)['errors'];checks.append(case+'_occurrence_order_detected')
    candidate=deepcopy(artifact);candidate['preparation']['occurrences'].pop();assert module.compare(candidate,case)['errors'];checks.append(case+'_occurrence_cardinality_detected')
    candidate=deepcopy(artifact);del candidate['preparation']['occurrences'][0]['expression'];assert module.compare(candidate,case)['errors'];checks.append(case+'_missing_nullable_field_detected')
    candidate=deepcopy(artifact);del candidate['admission']['bounds'];assert module.compare(candidate,case)['errors'];checks.append(case+'_missing_bounds_detected')
    candidate=deepcopy(artifact);candidate['admission']['bounds']['code_rows']=64;assert module.compare(candidate,case)['errors'];checks.append(case+'_bounds_decimal_type_detected')
    candidate=deepcopy(artifact);candidate['preparation']['lets'][0]['expression']=17;candidate['preparation']['symbol_domain']='OTHER_SYNTHETIC_UNSELECTED';candidate['preparation']['occurrences'][0]['source_expression']={'kind':'Name','name':'SYNTHETIC_UNSELECTED'}
    report=module.compare(candidate,case);assert not report['errors'];assert any(x['path']=='/preparation/lets/0/expression' and x['status']=='UNSELECTED_VALUE' for x in report['preparation_unselected_values']);checks.append(case+'_unknown_values_remain_explicitly_unselected')
observation={'status':'PASS_SYNTHETIC_SELECTED_PREPARATION_BOUNDS_NORMAL_MUTATIONS','check_count':len(checks),'checks':checks,'subject_outputs_or_producer_implementation_read':False,'executed':'Inspected pure stdlib comparator+golden copies only, no compiler/interpreter/model/SDK/native calls','unselected_metadata':'Synthetic placeholders exercise types/presence; they are never claimed as independently known actual raw values'}
(P/'TEST-OBSERVATION.json').write_text(json.dumps(observation,indent=2)+'\n')
print(json.dumps(observation))
