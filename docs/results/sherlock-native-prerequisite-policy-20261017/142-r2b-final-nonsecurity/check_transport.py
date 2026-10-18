from pathlib import Path
import json,re,hashlib,collections
D=Path(__file__).resolve().parent;m=json.loads((D/'INPUT-MEMBERSHIP.json').read_text());allowed={x['path']:x for x in m['pages']};raw=[json.loads(x) for x in (D/'AVAILABLE-TOOL-TRANSPORT.jsonl').read_text().splitlines()]; updates=raw;by={}
for x in updates:by[x['step_index']]=x
reads=[];bad=[]
for step,x in sorted(by.items()):
 info=x.get('tool_info',{});params=info.get('parameters',{});p=params.get('AbsolutePath'); row={'step_index':step,'state':x.get('state'),'tool_name':x.get('tool_name',info.get('name')),'parameters':params,'output_summary':info.get('output')};ok=row['tool_name']=='view_file' and set(params)=={'AbsolutePath'} and p in allowed
 if not ok:bad.append(row)
 if p in allowed:
  b=Path(p).read_bytes();s=info.get('output','');match=re.fullmatch(r'(\d+) lines, (\d+) bytes',s) if isinstance(s,str) else None;row.update({'path':p,'file_id':allowed[p]['file_id'],'expected_bytes':len(b),'expected_summary_lines':b.count(b'\n')+1,'summary_counts_match':bool(match and int(match[1])==b.count(b'\n')+1 and int(match[2])==len(b))})
 reads.append(row)
done=[x for x in reads if x['state']=='DONE' and x.get('path') in allowed];counts=collections.Counter(x['path'] for x in done);missing=sorted(set(allowed)-set(counts));dupes={p:n for p,n in counts.items() if n!=1};pins=json.loads((D/'INPUT-MEMBERSHIP.json').read_text());changes=[]
for x in pins['files']:
 b=Path(x['path']).read_bytes()
 if len(b)!=x['bytes'] or hashlib.sha256(b).hexdigest()!=x['sha256']:changes.append(x['path'])
result={'status':'TERMINAL' if (D/'RESULT.json').exists() else 'LIVE_OBSERVATION','required_pages':len(allowed),'unique_tool_steps':len(by),'tool_updates':len(updates),'done_allowlisted_pages':len(counts),'all_required_done_once':not missing and not dupes,'missing_pages':missing,'duplicate_done_reads':dupes,'unexpected_tools_paths_parameters':bad,'all_done_summary_line_byte_counts_match':all(x.get('summary_counts_match') for x in done),'reads':reads,'selected_input_pins_changed':changes,'public_tool_response_limitation':'Actual AGY tool_info output contains line/byte summary, not full response text. Count/path summaries are mechanically checked; end-to-end body fidelity and semantic reading are not proved by summaries. Auditor scope and literal source-grounding checked separately.','native_authority':0,'subject_execution':False}
(D/'TOOL-READ-COVERAGE.json').write_text(json.dumps(result,indent=2)+'\n'); print(json.dumps({k:v for k,v in result.items() if k!='reads'},indent=2))
