# Existing same-fe noTools binary observer, contrast+B0 manual part2; metadata-only. Root sole caller.
from pathlib import Path
import collections,datetime,hashlib,json,subprocess,tempfile,time,re,os,signal,threading
D=Path(__file__).resolve().parent;manifest=json.loads((D/'MEMBERSHIP.json').read_text());EXPECTED_CID='fe7486d5-c674-4f04-9f52-ef41e1eff966'
selection=json.loads((D/'ROOT-SELECTION.json').read_text())
assert selection['authorized_one_provider_call'] is True
assert selection['model']=='gemini-4-argon-high'
assert selection['conversation']==EXPECTED_CID
assert selection['cwd']==manifest['cwd']
required={str(D/name) for name in ['PROMPT.proposed.txt','MEMBERSHIP.json','CALL.proposed.json','run_once_binary.py']}
assert required <= {p['path'] for p in selection['pins']}
assert not (D/'LAUNCH.json').exists(), 'One selection, no retry'

def write(n,x):
 payload=(json.dumps(x,indent=2,ensure_ascii=False)+'\n').encode();temporary=None
 try:
  with tempfile.NamedTemporaryFile(mode='wb',prefix='.'+n+'.',suffix='.tmp',dir=D,delete=False) as stream:
   temporary=Path(stream.name);stream.write(payload)
  os.replace(temporary,D/n)
 finally:
  if temporary is not None:temporary.unlink(missing_ok=True)
def pincheck():
 rows=[];prompt=(D/'PROMPT.proposed.txt').read_bytes()
 for f in manifest['inline_inputs']:
  raw=Path(f['path']).read_bytes();rows.append({'path':f['path'],'match':len(raw)==f['bytes'] and hashlib.sha256(raw).hexdigest()==f['sha256'] and prompt[f['inline_byte_start']:f['inline_byte_end_exclusive']]==raw})
 for f in manifest['external_pinned_inputs']:
  raw=Path(f['path']).read_bytes();rows.append({'path':f['path'],'match':len(raw)==f['bytes'] and hashlib.sha256(raw).hexdigest()==f['sha256']})
 for f in selection['pins']:
  raw=Path(f['path']).read_bytes();rows.append({'path':f['path'],'match':len(raw)==f['bytes'] and hashlib.sha256(raw).hexdigest()==f['sha256']})
 f=manifest['prompt'];rows.append({'path':'exact complete inline prompt','match':len(prompt)==f['bytes'] and hashlib.sha256(prompt).hexdigest()==f['sha256'] and prompt.endswith(b'END-WITNESS-CONTRAST-B0-PART-02\n') and len(prompt)<=manifest['inline_budget_bytes']})
 return {'files':rows,'all_match':all(x['match'] for x in rows)}
pre=pincheck();write('PRECALL-PIN-CHECK.json',pre);assert pre['all_match'];wire=(json.dumps({'event':'user','message':{'content':(D/'PROMPT.proposed.txt').read_bytes().decode('utf-8')}},ensure_ascii=False)+'\n').encode();(D/'INPUT-LF-NDJSON.bin').write_bytes(wire)
cmd=['/usr/bin/timeout', '--signal=TERM', '--kill-after=5s', '1320s', '/home/charl/.local/bin/agy', '--model', 'gemini-4-argon-high', '--mode', 'plan', '--conversation', EXPECTED_CID, '--input-format', 'stream-json', '--output-format', 'stream-json', '--print-timeout', '1200s'];cwd=manifest['cwd'];assert Path(cwd).is_dir();start=time.monotonic();startutc=datetime.datetime.now(datetime.timezone.utc).isoformat();counts=collections.Counter();types=collections.Counter();models=[];tools=[];users=[];public_errors=[];visible=[];terminal=None;terminals=0;parseerrors=0;stdinerror=None
observer_violations=[]; returned_conversation_ids=set()
private={'thought','thoughts','thinking','reasoning','analysis','private_thought','private_thoughts','thinking_delta','reasoning_delta','thought_delta'}
def filt(x):
 if isinstance(x,dict):return {k:filt(v) for k,v in x.items() if k.lower() not in private}
 if isinstance(x,list):return [filt(y) for y in x]
 return x
stop_lock=threading.Lock()
def observer_stop(reason,detail):
 with stop_lock:
  observer_violations.append({'reason':reason,'detail':detail});write('OBSERVER-STOP.json',{'violations':observer_violations,'action':'kill provider process group; no retry'})
  try:os.killpg(p.pid,signal.SIGKILL)
  except ProcessLookupError:pass
def retry_marker(value):
 return isinstance(value,str) and bool(re.search(r'stream[ _-]?idle|idle[ _-]?timeout|retries remaining|retrying request|retrying in |retry[_ -]?attempt|retry[_ -]?count',value,re.I))
with (D/'PROVIDER.stderr.bin').open('xb') as err,(D/'AVAILABLE-TRANSPORT.jsonl').open('xb') as tr,(D/'VISIBLE.jsonl').open('xb') as vis:
 def emit(f,x):f.write((json.dumps(x,ensure_ascii=False)+'\n').encode());f.flush()
 p=subprocess.Popen(cmd,cwd=cwd,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,start_new_session=True)
 def capture_stderr():
  for rawerr in p.stderr:
   err.write(rawerr);err.flush()
   if retry_marker(rawerr.decode('utf-8',errors='replace')):observer_stop('provider internal idle/retry on stderr','public stderr marker');break
 stderr_thread=threading.Thread(target=capture_stderr,daemon=True);stderr_thread.start()
 write('LAUNCH.json',{'pid':p.pid,'command':cmd,'start_utc':startutc,'requested_model':'gemini-4-argon-high','requested_effort':'HIGHpinnedslug','original_reviewer_cwd':cwd,'conversation_argument':EXPECTED_CID,'source_author_of_reviewed_guard':False,'candidate_author_CID':None,'source_author_of_reviewed_output_test':False,'actual_role':'same_actual_nonauthor_witness_contrast_B0_part2_source_three_operations_final','input_wire_bytes':len(wire),'input_wire_sha256':hashlib.sha256(wire).hexdigest(),'one_manual_call_no_auto_restart':True,'CLI1200s_GNU1320s_TERMkillafter5s':True,'binary_stdin_stdout_stderr_transport':True,'capture':'Only actualpublicinit/user/tool/visible/error metadata+terminal; hidden thought/context text discarded; no rawprivatepayloadcapture','subject_jobs_selected':0})
 try:p.stdin.write(wire);p.stdin.close()
 except BrokenPipeError as e:stdinerror=type(e).__name__
 for raw in p.stdout:
  try:v=json.loads(raw)
  except Exception:parseerrors+=1;continue
  if not isinstance(v,dict):parseerrors+=1;continue
  event=v.get('event');counts[str(event)]+=1
  if retry_marker(v.get('error')):observer_stop('provider internal idle/retry error event','public error marker');break
  cids=[v.get('conversation_id')]
  for key in ('init','step_update','result'):
   if isinstance(v.get(key),dict):cids.append(v[key].get('conversation_id'))
  cids=[x for x in cids if x]
  returned_conversation_ids.update(cids)
  if any(cid!=EXPECTED_CID for cid in returned_conversation_ids) or len(returned_conversation_ids)>1:
   observer_violations.append({'event':event,'reason':'returned CID differs from selected independent reviewer','actual_CIDs':cids,'expected_reviewer_CID':EXPECTED_CID});write('OBSERVER-STOP.json',{'violations':observer_violations,'action':'kill provider process group; no retry'});os.killpg(p.pid,signal.SIGKILL);break
  if cids:write('LIVE-CONTEXT.json',{'expected_reviewer_CID':EXPECTED_CID,'returned_CIDs':sorted(returned_conversation_ids),'last_event':event,'same_selected_reviewer_CID':True})
  if event=='init':
   init=v.get('init',{});sel={k:init[k] for k in ['model','model_slug','model_name','effort','mode','permission_mode','cwd'] if k in init};models.append(sel);emit(tr,{'event':'init_available_selection','selection':sel,'conversation_id':v.get('conversation_id')})
   if init.get('model')!='gemini-4-argon-high' or init.get('cwd')!=cwd:
    observer_violations.append({'event':'init','reason':'selected model or original reviewer cwd mismatch','selection':sel});write('OBSERVER-STOP.json',{'violations':observer_violations,'action':'kill provider process group; no retry'});os.killpg(p.pid,signal.SIGKILL);break
  elif event=='step_update':
   u=v.get('step_update',{});typ=u.get('step_type');types[str(typ)]+=1
   if u.get('conversation_id'):returned_conversation_ids.add(u['conversation_id'])
   if typ=='user_input':users.append(filt(u));emit(tr,{'event':'available_user_input','step_update':filt(u)})
   if typ=='tool' or 'tool_info' in u:
    tools.append(filt(u));emit(tr,{'event':'available_tool_step','step_update':filt(u)})
    observer_stop('ANY_TOOL_FORBIDDEN_DATA_ONLY_REVIEW_FOLLOWUP',{'step_index':u.get('step_index'),'tool_name':u.get('tool_name'),'tool_info':filt(u.get('tool_info'))});break
   if typ=='error_message':
    x={k:u[k] for k in ['state','step_index','conversation_id','error_type','code','status','retry_count','retry_attempt','timeout_seconds'] if k in u};x['available_keys']=list(u);s=' '.join(u[k] for k in ['message','error','text','text_delta'] if isinstance(u.get(k),str));x['public_transport_keyword_flags']={w:bool(re.search(w,s,re.I)) for w in ['retry','idle','timeout','stream','quota']};public_errors.append(x);emit(tr,{'event':'public_error_metadata','metadata':x})
   if typ=='error_message':observer_stop('provider public error event; no retry',{'step_index':u.get('step_index'),'error_type':u.get('error_type')});break
   if retry_marker(u.get('error')):observer_stop('provider internal idle/retry step error','public error marker');break
   if typ=='checkpoint':emit(tr,{'event':'checkpoint_metadata_only','state':u.get('state'),'step_index':u.get('step_index'),'available_keys':list(u)})
   if typ=='agent_response':
    for field in ['text_delta','text','response']:
     if isinstance(u.get(field),str) and u[field]:
      x={'step_index':u.get('step_index'),'state':u.get('state'),'public_field':field,'text':u[field]};visible.append(x);emit(vis,x)
  elif event=='result':
   z=v.get('result',{});terminals+=1;terminal={k:z[k] for k in ['status','response','error','duration_seconds','num_turns','usage','conversation_id','model','model_slug','effort'] if k in z}
 code=p.wait();stderr_thread.join(timeout=2)
post=pincheck();write('POSTCALL-PIN-CHECK.json',post);response=terminal.get('response') if terminal else None
if isinstance(response,str) and response:(D/'FINAL-RESPONSE.txt').write_bytes(response.encode());(D/'REPORT.md').write_bytes((response+'\n').encode())
status={'returned_conversation_ids':sorted(returned_conversation_ids),'expected_reviewer_CID':EXPECTED_CID,'same_selected_reviewer_CID':returned_conversation_ids=={EXPECTED_CID},'actual_exit':code,'start_utc':startutc,'end_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'elapsed_seconds':time.monotonic()-start,'requested_model':'gemini-4-argon-high','requested_effort':'HIGHactualpinnedslug','returned_init_selection':models,'backendcanonical_attestation':False,'event_counts':dict(counts),'step_type_counts':dict(types),'terminal_count':terminals,'terminal':terminal,'stream_parse_error_count':parseerrors,'stdin_error':stdinerror,'available_user_input_events':users,'available_tool_steps':tools,'available_public_error_metadata':public_errors,'visible_public_fields_captured':sorted(set(x['public_field'] for x in visible)),'final_response_bytes':len(response.encode()) if isinstance(response,str) else 0,'all_selected_input_pins_unchanged':post['all_match'],'observer_violations':observer_violations,'one_manual_provider_launch':True,'subject_jobs_selected':0,'tests_run':False,'source_author_of_reviewed_guard':False,'candidate_author_CID':None,'source_author_of_reviewed_output_test':False,'actual_role':'same_actual_nonauthor_witness_contrast_B0_part2_source_three_operations_final'}
write('RESULT.json',status);print(json.dumps({k:v for k,v in status.items() if k not in ['terminal','available_user_input_events','available_tool_steps']}),flush=True)

# No tools are permitted; inline completeness is pinned locally, readable author coverage remains required.
write('DELIVERY-READBACK.json',{'complete_inline_input_pins_current':post['all_match'],'inline_body_count':len(manifest['inline_inputs']),'inline_prompt_bytes':manifest['prompt']['bytes'],'actual_tool_steps':len(tools),'same_selected_reviewer_CID':returned_conversation_ids=={EXPECTED_CID},'provider_user_input_verbatim_readback_available':False,'reviewer_explicit_readable_coverage_required':True,'no_tool_or_subject_execution_authorized':True,'subject_jobs_selected':0})
if tools or observer_violations or code!=0 or not post['all_match'] or terminals!=1 or not terminal or terminal.get('status')!='SUCCESS' or returned_conversation_ids!={EXPECTED_CID}:
 raise SystemExit('Review-followup transport incomplete/failed: no adoption/materialization; no retry')
