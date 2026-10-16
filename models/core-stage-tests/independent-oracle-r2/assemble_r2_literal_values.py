"""Fill only r1's twelve attachment gaps with independently derived literals.

Reads r1 owned data and three supplied premise/review documents only.
Does not import or invoke Moriarty, and does not compare any runtime output.
"""
from pathlib import Path
import copy
import hashlib
import json

OUT = Path(__file__).parent
B = OUT.parent
R1 = B / 'mori-test-stages-independent-normal-oracle-r1'

def digest(data): return hashlib.sha256(data).hexdigest()
def read(name): return json.loads((R1 / name).read_text())
def write(name, data):
    (OUT / name).write_text(json.dumps(data, indent=2, sort_keys=True) + '\n')
def u(n): return {'type':'UInt','value':str(n)}
def b(v): return {'type':'Bool','value':v}

# Contents derive solely from r1's normative/scenario arithmetic. Category
# numbers select attachment only, as explicitly constrained by C1/C5.
replacements = {
 'two-deliveries-start': [(0,6,'Transfer',u(7),'Source statement 0 has literal transfer amount7.'),
                          (1,4,'Next',u(1),'Source statement 1 sets phase to literal1.')],
 'two-deliveries-observed': [],
 'two-deliveries-approved': [(0,10,'Await',b(True),'approval-true supplies ok:true; guard is ok.'),
                            (1,6,'Transfer',u(3),'Source statement 3 has literal transfer amount3.')],
 'two-deliveries-complete': [(0,10,'Await',b(True),'receipt-true supplies seen:true; guard is seen.'),
                            (1,4,'Next',u(2),'Source statement 5 sets phase to literal2.'),
                            (2,5,'Emit',u(2),'Source statement 6 emits current phase2.')],
 'quoted-fee-start': [(0,1,'Let',u(5),'Input amount4 plus literal1 yields captured payable5.')],
 'quoted-fee-complete': [(0,10,'Await',b(True),'quote-true supplies agreed:true; guard is agreed.'),
                        (1,7,'Fee',u(5),'Captured payable5 is the source fee amount.'),
                        (2,4,'Next',u(5),'Captured payable5 becomes total.'),
                        (3,5,'Emit',u(5),'Source emit expression is captured payable5.')],
}

old_to_new = []
reports = []
for stem, slots in replacements.items():
    old_name = stem + '-manifest-input-caps.partial.json'
    new_name = stem + '.json'
    original = read(old_name)
    candidate = copy.deepcopy(original)
    for index, category, kind, value, derivation in slots:
        pointer = f'/segment/execution/steps/{index}/value'
        old = original['segment']['execution']['steps'][index]['value']
        candidate['segment']['execution']['steps'][index]['value'] = value
        old_to_new.append({
          'old_file':old_name,'old_file_sha256':digest((R1/old_name).read_bytes()),
          'new_file':new_name,'pointer':pointer,'old':old,'new':value,
          'actual_source_statement_index':original['segment']['execution']['steps'][index]['index'],
          'actual_statement_kind':kind,'reviewed_opus_category':category,
          'premise':'P-TRACE-PROJ/draft','premise_use':'selection/attachment only',
          'independent_value_derivation':derivation,'outside_reviewed_categories':False,
        })
    write(new_name,candidate)
    reports.append(new_name)

for name in ['input-sha256.json','source-premise-sha256.json','canonical-digests.json',
             'two-deliveries-program-preimage.json','two-deliveries-plan-preimage.json',
             'quoted-fee-program-preimage.json','quoted-fee-plan-preimage.json']:
    (OUT/name).write_bytes((R1/name).read_bytes())
(OUT/'R1-DERIVATION-PRESERVED.md').write_bytes((R1/'DERIVATION.md').read_bytes())
(OUT/'R1-NORMAL-ACCOUNTING-PRESERVED.json').write_bytes((R1/'normal-accounting-and-work.json').read_bytes())

write('old-to-new-gap-map.json',{
 'format':'moriarty-independent-value-gap-map/1',
 'old_freeze_sha256':digest((R1/'INDEPENDENT-FREEZE.json').read_bytes()),
 'field_replacements':old_to_new,'replaced_field_count':12,
 'all_actual_kinds':['Let','Next','Emit','Transfer','Fee','Await'],
 'outside_reviewed_categories':[],
 'other_report_fields':'Retained directly from r1; the assembly assigns only the 12 pointers listed.',
 'execution':'UNRUN','runtime_output_comparisons':'UNRUN'})

premises = [
 'core-trace-contract-candidate-r1/CORE-TRACE-REPORT-CONTRACT.md',
 'core-trace-contract-review-muse-r1/RESULT.md',
 'core-trace-contract-review-opus-r1/RESULT.md',
]
write('projection-premise-sha256.json',{name:digest((B/name).read_bytes()) for name in premises})
write('actual-call-premises.json',{
 'root_input_only_projection_sha256':'09848058f2d1bf32d80d6176ae764b48fd551f6aeaa2c3a676a37d8447bd1ee2',
 'profile':'moriarty-tests/2','route':'Core/2 stage start/resume only',
 'cases':[
  {'name':'two-deliveries','source':'two-deliveries.mori','scenario':'two-deliveries-scenario.json',
   'action':'deliver','logical_cap':'16','events':['approval-false.json','approval-true.json','receipt-true.json']},
  {'name':'quoted-fee','source':'quoted-fee.mori','scenario':'quoted-fee-scenario.json',
   'action':'charge','logical_cap':None,'events':['quote-true.json']}],
 'resume_cli_cap':None,'resume_optional_action_assertion':None,
 'resume_input':'Immediately preceding waiting report continuation',
 'metered_core1':'OUT_OF_SCOPE','backend_trace':'OUT_OF_SCOPE',
 'execution':'UNRUN','comparisons':'UNRUN'})
write('content-status.json',{
 'status':'COMPLETE_CONTENT_UNDER_DISCLOSED_PROJECTION_PREMISE_REVIEW_PENDING',
 'complete_normal_report_candidates':reports,'remaining_report_placeholder_fields':[],
 'trace_gap_disposition':'12 concrete r1 value fields supplied under P-TRACE-PROJ/draft; no normative gap closed',
 'draft_gap_records_preserved':14,'source_result_reviews':'PENDING',
 'root_full_freeze_review':'PENDING','comparison_admission':'NOT_GRANTED',
 'execution':'UNRUN','comparisons':'UNRUN','native_authority':0})
