"""Pure normal golden-copy/mutation tests; never actual subject output."""
from copy import deepcopy
import importlib.util,json
from pathlib import Path
import sys
sys.dont_write_bytecode=True
P=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('independent_r115_compare',P/'compare_expected.py')
module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
checks=[]
for case,e in module.golden().items():
    actual={'source':deepcopy(e['source']),'preparation':deepcopy(e['preparation']),'admission':{'program_view':deepcopy(e['program_view'])}}
    assert not module.compare(actual,case)['errors'];checks.append(case+'_complete_populated_selected_match')
    for field in sorted(module.FIELDS):
        changed=deepcopy(actual);v=changed['admission']['program_view']
        if field=='entry':v[field]='1'
        elif field=='expressions':v[field][0]={'Literal':{'constant':'0'}}
        elif field=='occurrences':v[field][0]['owner_is_function']=True
        elif field=='occurrence_paths':v[field][0]='1'
        elif field=='children':v[field][0]='2'
        elif field=='functions':v[field][0]['body']='0'
        elif field=='parameters':v[field][0]['sort']='UInt'
        elif field=='arms':v[field][0]['body']='0'
        elif field=='patterns':v[field].reverse()
        elif field=='constants':v[field][0]['U128']['n']='9'
        elif field=='constant_fields':v[field].append('0')
        elif field=='names':v[field].reverse()
        assert module.compare(changed,case)['errors'],(case,field);checks.append(case+'_changed_'+field+'_detected')
    changed=deepcopy(actual);del changed['admission']['program_view']['constant_fields'];assert module.compare(changed,case)['errors'];checks.append(case+'_missing_empty_table_detected')
    changed=deepcopy(actual);changed['preparation']['provenance']['expressions'][0],changed['preparation']['provenance']['expressions'][1]=changed['preparation']['provenance']['expressions'][1],changed['preparation']['provenance']['expressions'][0];assert module.compare(changed,case)['errors'];checks.append(case+'_provenance_order_detected')
    changed=deepcopy(actual);changed['preparation']['provenance']['expressions'][0]['span']['start']+=1;assert module.compare(changed,case)['errors'];checks.append(case+'_span_detected')
    changed=deepcopy(actual);changed['preparation']['provenance']['expressions'][0]['span']['start']=str(changed['preparation']['provenance']['expressions'][0]['span']['start']);assert module.compare(changed,case)['errors'];checks.append(case+'_span_numeric_type_detected')
    changed=deepcopy(actual);changed['preparation']['types']['entry_type']='Tree';assert module.compare(changed,case)['errors'];checks.append(case+'_type_summary_detected')
    changed=deepcopy(actual);changed['preparation']['functions'][0]['parameters'][0]['name']='changed';assert module.compare(changed,case)['errors'];checks.append(case+'_source_declaration_detected')
    changed=deepcopy(actual);changed['source']['source_digest']='f'*64;assert module.compare(changed,case)['errors'];checks.append(case+'_source_digest_detected')
    changed=deepcopy(actual);changed['preparation']['provenance']['patterns'][1]['binders'][0]['name']='changed';assert module.compare(changed,case)['errors'];checks.append(case+'_pattern_source_binder_detected')
    changed=deepcopy(actual);changed['admission']['program_view']['children'].append(changed['admission']['program_view']['children'][0]);assert module.compare(changed,case)['errors'];checks.append(case+'_child_multiplicity_detected')
report={'status':'PASS_PURE_GOLDEN_COPY_AND_SELECTED_STATIC_FIELD_MUTATIONS','check_count':len(checks),'checks':checks,'actual_subject_data_used':False,'subject_execution':False,'scope':'Complete R112 selected interface against independently populated literal expectations; no actual parser/checker/bridge/owner/compiler/model/native calls'}
(P/'COMPARISON-CHECKS.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report))
