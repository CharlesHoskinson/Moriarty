from pathlib import Path
import collections,datetime,hashlib,json,subprocess,time,threading,os,signal
D=Path(__file__).parent;sha=lambda b:hashlib.sha256(b).hexdigest();F=json.loads((D/'REVIEW-FREEZE.json').read_text());M=json.loads((D/'INPUT-MANIFEST.json').read_text());W=Path('/home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016')
def write(name,x):(D/name).write_text(json.dumps(x,indent=2)+'\n')
def pins():
 rows=[]
 for x in M['units']:
  b=Path(x['source']).read_bytes();selected=Path(x['snapshot']).read_bytes();rows.append({'label':x['label'],'source_match':len(b)==x['original_bytes'] and sha(b)==x['original_sha256'],'snapshot_match':len(selected)==x['selected_bytes'] and sha(selected)==x['selected_sha256']})
 return {'files':rows,'all_match':all(v['source_match'] and v['snapshot_match'] for v in rows),'no_subject_rerun':True}
assert not (D/'LAUNCH.json').exists(), 'one request only; no retry';pre=pins();write('PRECALL-PINS.json',pre);assert pre['all_match'];assert sha((D/'PROMPT.txt').read_bytes())==F['prompt_sha256'];assert sha((D/'INPUT-MANIFEST.json').read_bytes())==F['manifest_sha256']
CMD=['/usr/bin/timeout','--signal=TERM','--kill-after=10s','600s','/home/charl/.local/bin/agy','--model','gemini-4-argon-high','--mode','plan','--input-format','stream-json','--output-format','stream-json','--print-timeout','480s'];C=D/'review-session';C.mkdir(exist_ok=False);start=time.monotonic();startutc=datetime.datetime.now(datetime.timezone.utc).isoformat();counts=collections.Counter();types=collections.Counter();models=[];tools=[];terminal=None;errors=0;stops=[];lock=threading.Lock()
p=subprocess.Popen(CMD,cwd=C,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,start_new_session=True)
def stop(kind,source,literal=''):
 with lock:
  if p.poll() is not None:return
  first=not stops;stops.append({'kind':kind,'source':source,'literal_error_marker':literal[:2000],'elapsed_seconds':time.monotonic()-start});write('STOP-EVENTS.json',{'events':stops,'no_restart':True})
 if first:
  try:os.killpg(p.pid,signal.SIGTERM)
  except ProcessLookupError:return
  def force():
   if p.poll() is None:
    try:os.killpg(p.pid,signal.SIGKILL)
    except ProcessLookupError:pass
  timer=threading.Timer(10,force);timer.daemon=True;timer.start()
def marker(value,source):
 if isinstance(value,str) and any(m in value.lower() for m in ['stream idle','stream_idle','idle timeout','idle_timeout','retries remaining','retrying request','retrying in ']):stop('OBSERVED_PROVIDER_INTERNAL_IDLE_OR_RETRY',source,value)
def stderr_reader():
 with (D/'PROVIDER.stderr').open('xb') as stream:
  for raw in p.stderr:stream.write(raw);stream.flush();marker(raw.decode('utf-8',errors='replace'),'provider_stderr')
thread=threading.Thread(target=stderr_reader,daemon=True);thread.start();outer=None # Actual GNU timeout wrapper provides the 600-second outer bound
write('LAUNCH.json',{'pid':p.pid,'command':CMD,'start_utc':startutc,'cwd':str(C),'role':'NONSECURITY_COMPILER_EVIDENCE_RECORD_FUNCTIONAL_RESULT_REVIEWER','requested_primary':'Opus5.5HIGH','role':'NONSECURITY_COMPILER_EVIDENCE_RECORD_FUNCTIONAL_RESULT_REVIEWER','requested_primary':'claude-opus-5-5','requested_model':'gemini-4-argon-high','requested_effort':'high in pinnedmodelslug','fallback_authority':'standing observed Opusweeklyquota outage; no probe','fresh_context':True,'source_freeze_sha256':F.get('source_freeze_sha256'),'prompt_bytes':F['prompt_bytes'],'prompt_sha256':F['prompt_sha256'],'manifest_sha256':F['manifest_sha256'],'provider_print_timeout_seconds':480,'GNU_outer_deadline_seconds':600,'termination_grace_seconds':10,'stderr_capture':'binarypipe -> binaryfile, decodeonlyformarkers','no_autorestart_proxy_authswitch_retry':True,'native_authority':0});print(json.dumps({'LAUNCHED':p.pid,'launch':str(D/'LAUNCH.json'),'prompt_bytes':F['prompt_bytes'],'print_timeout':'480s','outer_seconds':600}),flush=True)
p.stdin.write((json.dumps({'event':'user','message':{'content':(D/'PROMPT.txt').read_text()}},ensure_ascii=False)+'\n').encode());p.stdin.close()
with (D/'AVAILABLE-TOOL-TRANSPORT.jsonl').open('x') as transport:
 for raw in p.stdout:
  try:v=json.loads(raw)
  except Exception:errors+=1;continue
  if not isinstance(v,dict):errors+=1;continue
  event=v.get('event');counts[str(event)]+=1;marker(v.get('error'),'provider_error_event')
  if event=='init':
   init=v.get('init',{});models.append({k:init[k] for k in ['model','model_slug','model_name','effort','model_selection','permission_mode','mode','cwd','tools'] if k in init})
  elif event=='step_update':
   u=v.get('step_update',{});typ=u.get('step_type');types[str(typ)]+=1;marker(u.get('error'),'provider_step_error')
   if typ=='tool' or 'tool_info' in u:
    safe={k:u[k] for k in ['conversation_id','step_index','state','step_type','tool_name','duration_seconds','tool_info','error'] if k in u};tools.append(safe);transport.write(json.dumps(safe,ensure_ascii=False)+'\n');transport.flush()
  elif event=='result':
   result=v.get('result',{});marker(result.get('error'),'provider_terminal_error');terminal={k:result[k] for k in ['status','response','error','duration_seconds','num_turns','usage','conversation_id','model','model_slug','model_name','effort','model_selection'] if k in result}
code=p.wait();thread.join(timeout=2);report=(terminal or {}).get('response','');report=report if isinstance(report,str) else json.dumps(report,ensure_ascii=False);(D/'REPORT.md').write_text(report);post=pins();write('POSTCALL-PINS.json',post);errbytes=(D/'PROVIDER.stderr').read_bytes();r={'actual_exit':code,'start_utc':startutc,'end_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'elapsed_seconds':time.monotonic()-start,'role':'NONSECURITY_COMPILER_EVIDENCE_RECORD_FUNCTIONAL_RESULT_REVIEWER','requested_primary':'claude-opus-5-5','requested_model':'gemini-4-argon-high','requested_effort':'high in pinnedmodelslug','returned_init_selection':models,'backendcanonical_weights_attestation':False,'event_counts':dict(counts),'step_type_counts':dict(types),'terminal':terminal,'available_tool_steps':tools,'stream_parse_errors':errors,'report_bytes':len(report.encode()),'report_sha256':sha(report.encode()),'stderr_bytes':len(errbytes),'stderr_sha256':sha(errbytes),'stderr_encoding':'exactbinarycapture; decodeUTF8withreplacementonlyformarkers','stop_events':stops,'provider_print_timeout_seconds':480,'GNU_outer_deadline_seconds':600,'termination_grace_seconds':10,'no_autorestart_or_secondprovider_call':True,'all_input_pins_unchanged':post['all_match'],'tail_sentinel_present':'END-R122-COMPILER-EVIDENCE-RECORD' in report,'private_thought_text_or_rawproviderstream_retained':False,'source_compiler_test_SDK_native_W_Git_actions_by_controller':'NONE; immutable sourcefile reads/hashinspection; no subject import/execution, R-only outputs and oneproviderreview only','native_authority':0};write('RESULT.json',r);print(json.dumps({k:r[k] for k in ['actual_exit','elapsed_seconds','report_bytes','report_sha256','event_counts','step_type_counts','stop_events','all_input_pins_unchanged']}),flush=True)
