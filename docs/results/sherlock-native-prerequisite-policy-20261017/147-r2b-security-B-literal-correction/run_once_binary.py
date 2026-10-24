from pathlib import Path
import collections,datetime,hashlib,json,subprocess,tempfile,time,re,os,signal
D=Path(__file__).resolve().parent;manifest=json.loads((D/'INPUT-MEMBERSHIP.json').read_text())
def write(n,x):(D/n).write_bytes((json.dumps(x,indent=2,ensure_ascii=False)+'\n').encode())
def pincheck():
 rows=[]
 for x in manifest['files']:
  b=Path(x['path']).read_bytes();rows.append({'path':x['path'],'match':len(b)==x['bytes'] and hashlib.sha256(b).hexdigest()==x['sha256']})
 return {'files':rows,'all_match':all(x['match'] for x in rows)}
pre=pincheck();write('PRECALL-PIN-CHECK.json',pre);assert pre['all_match'];wire=(json.dumps({'event':'user','message':{'content':(D/'PROMPT.txt').read_bytes().decode('utf-8')}},ensure_ascii=False)+'\n').encode();(D/'INPUT-LF-NDJSON.bin').write_bytes(wire)
cmd=['/usr/bin/timeout','--signal=TERM','--kill-after=5s','600s','/home/charl/.local/bin/agy','--conversation','a762f936-5972-4115-ab19-b833fd3d78f7','--model','gemini-4-argon-high','--mode','plan','--input-format','stream-json','--output-format','stream-json','--print-timeout','480s'];cwd=tempfile.mkdtemp(prefix='moriarty-r147-reviewB-correction-');start=time.monotonic();startutc=datetime.datetime.now(datetime.timezone.utc).isoformat();counts=collections.Counter();types=collections.Counter();models=[];tools=[];users=[];public_errors=[];visible=[];terminal=None;terminals=0;parseerrors=0;stdinerror=None
observer_stop=None
private={'thought','thoughts','thinking','reasoning','analysis','private_thought','private_thoughts','thinking_delta','reasoning_delta','thought_delta'}
def filt(x):
 if isinstance(x,dict):return {k:filt(v) for k,v in x.items() if k.lower() not in private}
 if isinstance(x,list):return [filt(y) for y in x]
 return x
with (D/'PROVIDER.stderr.bin').open('xb') as err,(D/'AVAILABLE-TRANSPORT.jsonl').open('xb') as tr,(D/'VISIBLE.jsonl').open('xb') as vis:
 def emit(f,x):f.write((json.dumps(x,ensure_ascii=False)+'\n').encode());f.flush()
 p=subprocess.Popen(cmd,cwd=cwd,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=err,start_new_session=True)
 write('LAUNCH.json',{'pid':p.pid,'command':cmd,'start_utc':startutc,'requested_model':'gemini-4-argon-high','requested_effort':'HIGHpinnedslug','fresh_cwd':cwd,'correction_not_fresh_vote':True,'actual_role':'existing_designated_security_reviewer_B_actual_owner_literal_correction_not_fresh_vote','input_wire_bytes':len(wire),'input_wire_sha256':hashlib.sha256(wire).hexdigest(),'one_manual_call_no_auto_restart':True,'CLI480s_GNU600s_TERMkillafter5s':True,'binary_stdin_stdout_stderr_transport':True,'capture':'Only actualpublicinit/user/tool/visible/error metadata+terminal; hidden thought/context text discarded; no rawprivatepayloadcapture','native_authority':0})
 try:p.stdin.write(wire);p.stdin.close()
 except BrokenPipeError as e:stdinerror=type(e).__name__
 for raw in p.stdout:
  try:v=json.loads(raw)
  except Exception:parseerrors+=1;continue
  if not isinstance(v,dict):parseerrors+=1;continue
  event=v.get('event');counts[str(event)]+=1
  if event=='init':
   init=v.get('init',{});sel={k:init[k] for k in ['model','model_slug','model_name','effort','mode','permission_mode','cwd'] if k in init};models.append(sel);emit(tr,{'event':'init_available_selection','selection':sel})
  elif event=='step_update':
   u=v.get('step_update',{});typ=u.get('step_type');types[str(typ)]+=1
   if typ=='user_input':users.append(filt(u));emit(tr,{'event':'available_user_input','step_update':filt(u)})
   if typ=='tool' or 'tool_info' in u:
    tools.append(filt(u));emit(tr,{'event':'available_tool_step','step_update':filt(u)})
    info=u.get('tool_info',{});params=info.get('parameters',{});name=u.get('tool_name',info.get('name'))
    if True:
     observer_stop={'reason':'any_tool_forbidden_in_inline_owner_correction','step_index':u.get('step_index'),'tool_name':name,'parameters':params}
     write('OBSERVER-STOP.json',observer_stop)
     os.killpg(p.pid,signal.SIGTERM)
     break
   if typ=='error_message':
    x={k:u[k] for k in ['state','step_index','conversation_id','error_type','code','status','retry_count','retry_attempt','timeout_seconds'] if k in u};x['available_keys']=list(u);s=' '.join(u[k] for k in ['message','error','text','text_delta'] if isinstance(u.get(k),str));x['public_transport_keyword_flags']={w:bool(re.search(w,s,re.I)) for w in ['retry','idle','timeout','stream','quota']};public_errors.append(x);emit(tr,{'event':'public_error_metadata','metadata':x})
   if typ=='checkpoint':emit(tr,{'event':'checkpoint_metadata_only','state':u.get('state'),'step_index':u.get('step_index'),'available_keys':list(u)})
   if typ=='agent_response':
    for field in ['text_delta','text','response']:
     if isinstance(u.get(field),str) and u[field]:
      x={'step_index':u.get('step_index'),'state':u.get('state'),'public_field':field,'text':u[field]};visible.append(x);emit(vis,x)
  elif event=='result':
   z=v.get('result',{});terminals+=1;terminal={k:z[k] for k in ['status','response','error','duration_seconds','num_turns','usage','conversation_id','model','model_slug','effort'] if k in z}
 try:code=p.wait(timeout=5 if observer_stop else None)
 except subprocess.TimeoutExpired:
  os.killpg(p.pid,signal.SIGKILL);code=p.wait()
post=pincheck();write('POSTCALL-PIN-CHECK.json',post);response=terminal.get('response') if terminal else None
if isinstance(response,str) and response:(D/'FINAL-RESPONSE.txt').write_bytes(response.encode());(D/'REPORT.md').write_bytes((response+'\n').encode())
status={'actual_exit':code,'start_utc':startutc,'end_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'elapsed_seconds':time.monotonic()-start,'requested_model':'gemini-4-argon-high','requested_effort':'HIGHactualpinnedslug','returned_init_selection':models,'backendcanonical_attestation':False,'event_counts':dict(counts),'step_type_counts':dict(types),'terminal_count':terminals,'terminal':terminal,'stream_parse_error_count':parseerrors,'stdin_error':stdinerror,'available_user_input_events':users,'available_tool_steps':tools,'available_public_error_metadata':public_errors,'visible_public_fields_captured':sorted(set(x['public_field'] for x in visible)),'final_response_bytes':len(response.encode()) if isinstance(response,str) else 0,'all_selected_input_pins_unchanged':post['all_match'],'one_manual_provider_launch':True,'native_authority':0,'tests_run':False,'observer_stop':observer_stop,'correction_not_fresh_vote':True,'actual_role':'existing_designated_security_reviewer_B_actual_owner_literal_correction_not_fresh_vote'}
write('RESULT.json',status);print(json.dumps({k:v for k,v in status.items() if k not in ['terminal','available_user_input_events','available_tool_steps']}),flush=True)
