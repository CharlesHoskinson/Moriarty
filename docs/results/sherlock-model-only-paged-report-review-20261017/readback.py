from pathlib import Path
import json,hashlib,re
D=Path(__file__).parent
def sha(b):return hashlib.sha256(b).hexdigest()
def dump(n,o):(D/n).write_text(json.dumps(o,indent=2)+'\n')
R=json.loads((D/'RESULT.json').read_text());M=json.loads((D/'REVIEW-READSET.json').read_text())
tools=R['available_tool_steps'];allowed={x['path']:x for x in M['pages']};done={};violations=[]
for t in tools:
 i=t.get('tool_info',{});name=i.get('name',t.get('tool_name'));p=i.get('parameters',{});path=p.get('AbsolutePath') if isinstance(p,dict) else None
 if name!='view_file' or not isinstance(p,dict) or set(p)!={'AbsolutePath'} or path not in allowed:
  violations.append({'step_index':t.get('step_index'),'tool':name,'parameters':p});continue
 if t.get('state')=='DONE':
  output=i.get('output');m=re.fullmatch(r'\s*(\d+) lines, (\d+) bytes\s*',output) if isinstance(output,str) else None
  expected=allowed[path];nlines=len(Path(path).read_bytes().split(b'\n'))
  done.setdefault(path,[]).append({'step_index':t.get('step_index'),'literal_tool_output':output,'reported_lines':int(m[1]) if m else None,'reported_bytes':int(m[2]) if m else None,'expected_lines':nlines,'expected_bytes':expected['bytes'],'size_metadata_matches':bool(m and int(m[1])==nlines and int(m[2])==expected['bytes']),'error':t.get('error')})
missing=sorted(set(allowed)-set(done));all_metadata=all(any(x['size_metadata_matches'] and not x['error'] for x in rs) for rs in done.values())
raw=(D/'REPORT.md').read_bytes();opinion=None;parse=None
try:opinion=json.loads(raw)
except Exception as e:parse=type(e).__name__
coverage=not missing and not violations and all_metadata and len(done)==42
eligible=R['actual_exit']==0 and (R.get('terminal') or {}).get('status')=='SUCCESS' and coverage and R['all_input_pins_unchanged'] and not R['stop_events'] and isinstance(opinion,dict) and R['tail_sentinel_present']
out={'status':'COMPLETE_REVIEW_RECEIVED' if eligible else 'INCOMPLETE_OR_INVALID_REVIEW_NO_APPROVAL_CREDIT','actual_exit':R['actual_exit'],'terminal_status':(R.get('terminal') or {}).get('status'),'actual_elapsed_seconds':R['elapsed_seconds'],'returned_init_selection':[{k:v for k,v in x.items() if k in ['model','model_slug','effort','mode','permission_mode']} for x in R['returned_init_selection']],'requested_model':'gemini-4-argon-high','backend_weight_attestation':False,'report_bytes':len(raw),'report_sha256':sha(raw),'report_parse_error':parse,'output_limit_met':len(raw)<=4096,'expected_page_count':42,'distinct_exact_allowlisted_DONE_reads':len(done),'tool_updates':len(tools),'page_reads':done,'missing_pages':missing,'tool_violations':violations,'all_DONE_size_metadata_matches':all_metadata,'coverage_limit':'Actual DONE metadata reports full line/byte sizes for exact frozen page paths. Provider tool metadata does not expose a cryptographic receipt of internal model context; no stronger attestation claimed.','all_before_after_input_pins_match':R['all_input_pins_unchanged'],'provider_stop_events':R['stop_events'],'actual_opinion_literal':opinion,'no_subject_model_SDK_native_W_Git_execution':True,'R132':'CANCELLED_ABSTAIN_NO_APPROVAL unchanged; no coverage transferred','semantic_acceptance':'Reviewer opinion only; Root integration decision separate'}
out['format_deviation']='Raw provider report exceeds requested4096B; retained verbatim, no retry. This length deviation is distinct from complete page coverage and the literal semantic decision.' if len(raw)>4096 else None
out['line_count_convention']='view_file metadata counts newline-separated entries including final empty entry. Initial local splitlines-based check was corrected; all reported byte sizes matched throughout.'
dump('LITERAL-READBACK.json',out)
brief={'kind':'COORDINATOR_DERIVED_READBACK_NOT_REPLACEMENT_PROVIDER_OPINION','raw_opinion_path':'REPORT.md','raw_opinion_sha256':sha(raw),'raw_opinion_bytes':len(raw),'requested_limit_bytes':4096,'format_deviation':out['format_deviation'],'actual_model_selection':out['returned_init_selection'],'actual_seconds':R['elapsed_seconds'],'actual_exit':R['actual_exit'],'actual42_page_DONE_reads_complete':coverage,'all118_before_after_pins_match':R['all_input_pins_unchanged'],'sevenFieldAssessments':{c:{f:v.split(':',1)[0] for f,v in fs.items()} for c,fs in opinion.get('sevenFieldAssessments',{}).items()},'modelRecordDecision':opinion.get('modelRecordDecision'),'originalScenarioDecision':'APPROVE' if opinion.get('originalScenarioDecision','').startswith('APPROVE') else opinion.get('originalScenarioDecision'),'materialFindings':opinion.get('materialFindings'),'whole025Recommendation':opinion.get('whole025Recommendation'),'limitations':opinion.get('limitations'),'R132':'Cancelled ABSTAIN unchanged; no transferred approval'}
assert len((json.dumps(brief,indent=2)+'\n').encode())<=4096
dump('CONCISE-READBACK.json',brief)
files=['PROMPT.txt','PAGE-INDEX.txt','REVIEW-READSET.json','PAGE-COMPLETENESS.json','RECONSTRUCTION-PROOF.json','PREPARATION-RESULT.json','PRECALL-PINS.json','POSTCALL-PINS.json','LAUNCH.json','RESULT.json','REPORT.md','AVAILABLE-TOOL-TRANSPORT.jsonl','PROVIDER.stderr','LITERAL-READBACK.json','CONCISE-READBACK.json','prepare.py','run_once.py','readback.py','ORIGINAL-RECORD.json']
files += [str(p.relative_to(D)) for base in ['pages','bodies'] for p in sorted((D/base).glob('*')) if p.is_file()]
pub={'status':'PORTABLE_PACKAGE_MAP_NOT_PUBLISHED','suggested_publication_root':'docs/results/sherlock-model-only-paged-report-review-20261017','copy_rule':'Copy each source below to destination_relative unchanged; retain original absolute-path provenance inside immutable records. External source pins are historical locators, not mandatory directory dependencies for reviewing supplied page bodies.','files':[{'source':str(D/n),'destination_relative':n,'bytes':(D/n).stat().st_size,'sha256':sha((D/n).read_bytes())} for n in files],'provider_private_thought_or_raw_stream':'not retained','R132':'separate cancelled record; do not publish as approval','readset_closed':True}
dump('PUBLICATION-MAP.json',pub)
print(json.dumps({k:out[k] for k in ['status','actual_exit','actual_elapsed_seconds','report_bytes','report_sha256','distinct_exact_allowlisted_DONE_reads','missing_pages','tool_violations','all_DONE_size_metadata_matches','all_before_after_input_pins_match']},indent=2))
print(raw.decode())
