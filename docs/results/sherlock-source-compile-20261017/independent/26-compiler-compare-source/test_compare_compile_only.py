"""Fixed normal golden-copy and selected-cell mutation checks only."""
from copy import deepcopy
import importlib.util
import json
from pathlib import Path
import sys
sys.dont_write_bytecode=True
P=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('independent_compile_compare',P/'compare_compile_only.py')
module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
O=module.ORACLE

def golden(case):
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

def mutate(a,path,value):
    current=a
    for key in path[:-1]:current=current[key]
    current[path[-1]]=value

checks=[]
for case in ['quote','eligibility']:
    original=golden(case);saved=deepcopy(original)
    result=module.compare(original,case)
    assert not result['errors'] and not result['comparison_complete']
    assert original==saved and result['raw_program_view']==original['admission']['program_view']
    if case=='eligibility':assert len(result['alpha_mapping'])==4 and result['normalized_program_view']==module.load(O/'EXPECTED-ELIGIBILITY-ALPHA-PROGRAM-VIEW.json')
    checks.append(case+'_golden_exact_selected_fields_raw_retained')
    mutations=[('entry',['admission','program_view','entry'],'1'),('literal',['admission','program_view','constants',0,'U128','n'],'4'),('marker',['admission','program_view','constants',0,'U128','marker_free'],False),('owner',['admission','program_view','occurrences',1,'owner_is_function'],False),('path',['admission','program_view','occurrence_paths',0],'1'),('return_type',['admission','program_view','functions',0,'return_type'],'Bool' if case=='quote' else 'UInt'),('identity',['admission','identity','profile_id'],3),('layout',['admission','layout','bounds','code_rows'],'65'),('observation',['observation','evaluation_performed'],True),('source_digest',['source','source_digest'],'0'*64)]
    for name,path,value in mutations:
        candidate=deepcopy(original);mutate(candidate,path,value);assert module.compare(candidate,case)['errors'],(case,name)
        checks.append(case+'_changed_'+name+'_detected')
    for name,field in [('array_order','expressions'),('multiplicity','constants')]:
        candidate=deepcopy(original);rows=candidate['admission']['program_view'][field]
        if name=='array_order':rows[3],rows[4]=rows[4],rows[3]
        else:rows.append(deepcopy(rows[0]))
        assert module.compare(candidate,case)['errors'],(case,name)
        checks.append(case+'_'+name+'_detected')
    candidate=deepcopy(original);del candidate['admission']['program_view']['children'];assert module.compare(candidate,case)['errors'];checks.append(case+'_missing_table_detected')
    candidate=deepcopy(original);candidate['admission']['program_view']['parameters'].append({'name':'0','sort':'UInt'});assert module.compare(candidate,case)['errors'];checks.append(case+'_unexpected_parameter_detected')
# These exact metadata bindings come from independently supplied public R28
# declarations and target bytes, never from implementation or captured output.
artifact=golden('quote')
assert not module.compare(artifact,'quote',bindings=module.load(module.PUBLIC_BINDINGS))['errors'];checks.append('pinned_public_R28_metadata_bindings_match')
for role,path,value in [('config_digest',['config','file_digest'],'f'*64),('target_digest',['target','file_digest'],'f'*64),('config_value',['config','fuel','value'],'1001'),('action_index',['source','action_index'],'1'),('target_descriptor',['target','descriptor','kind'],'changed-public-descriptor')]:
    candidate=deepcopy(artifact);mutate(candidate,path,value);assert module.compare(candidate,'quote')['errors'];checks.append('public_binding_changed_'+role+'_detected')
candidate=deepcopy(artifact);candidate['config']['extra']='unexpected';assert module.compare(candidate,'quote')['errors'];checks.append('extra_config_member_detected')
# u128/usize owner values remain decimal strings, never float-coerced; integer
# JSON parsing also retains precision for separately declared wrapper metadata.
assert module.load(O/'inputs/execution-config.json')['fuel']['value']=='1000'
assert json.loads('340282366920938463463374607431768211455')==(1<<128)-1
report={'status':'PASS_SYNTHETIC_GOLDEN_COPY_AND_NORMAL_MUTATION_CHECKS','checks':checks,'check_count':len(checks),'production_or_subject_calls':0,'scope':'Selected exact/alpha ProgramView and declared compile-only predicates; public R28 metadata bindings and target bytes; no actual output or implementation read'}
(P/'TEST-OBSERVATION.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report))
