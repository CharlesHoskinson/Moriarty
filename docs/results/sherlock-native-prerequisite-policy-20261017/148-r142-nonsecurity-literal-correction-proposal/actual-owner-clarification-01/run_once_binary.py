from pathlib import Path
import collections,datetime,hashlib,json,subprocess,time,threading,os,signal
D=Path(__file__).parent;sha=lambda b:hashlib.sha256(b).hexdigest();F=json.loads((D/'FREEZE.json').read_text());M=json.loads((D/'INPUT-MEMBERSHIP.json').read_text());W=Path('/home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016')
def write(name,x):(D/name).write_text(json.dumps(x,indent=2)+'\n')
def pins():
 rows=[]
 for x in M['files']:
  b=Path(x['path']).read_bytes();rows.append({'path':x['path'],'match':len(b)==x['bytes'] and sha(b)==x['sha256']})
 return {'files':rows,'all_match':all(x['match'] for x in rows),'no_subject_rerun':True}
pre=pins();write('PRECALL-PINS.json',pre);assert pre['all_match'];assert sha((D/'PROMPT.txt').read_bytes())==F['prompt_sha256'];assert sha((D/'INPUT-MEMBERSHIP.json').read_bytes())==F['membership_sha256']
CMD=['/usr/bin/timeout','--signal=TERM','--kill-after=10s','600s','/home/charl/.local/bin/agy','--model','gemini-4-argon-high','--conversation',M['requested_conversation_id'],'--mode','plan','--input-format','stream-json','--output-format','stream-json','--print-timeout','480s'];C=D/'review-session';C.mkdir(exist_ok=False);start=time.monotonic();startutc=datetime.datetime.now(datetime.timezone.utc).isoformat();allowed_pages={x['path'] for x in M['pages']};counts=collections.Counter();types=collections.Counter();models=[];tools=[];conversation_ids=set();terminal=None;errors=0;stops=[];lock=threading.Lock()
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
write('LAUNCH.json',{'pid':p.pid,'command':CMD,'start_utc':startutc,'cwd':str(C),'requested_primary':'Opus5.5HIGH','requested_model':'gemini-4-argon-high','requested_effort':'high in pinnedmodelslug','fallback_authority':'standing observed Opusweeklyquota outage; no probe','fresh_context':False,'requested_conversation_id':M['requested_conversation_id'],'context_mode':'actual_R142_owner_conversation_clarification_not_fresh_fullsource_review','source_freeze_sha256':M['source_freeze_sha256'],'actual_role':'actual_prior_nonsecurity_owner_narrow_literal_clarification','prompt_bytes':F['prompt_bytes'],'prompt_sha256':F['prompt_sha256'],'manifest_sha256':F['membership_sha256'],'provider_print_timeout_seconds':480,'GNU_outer_deadline_seconds':600,'termination_grace_seconds':10,'stderr_capture':'binarypipe -> binaryfile, decodeonlyformarkers','no_autorestart_proxy_authswitch_retry':True,'native_authority':0});print(json.dumps({'LAUNCHED':p.pid,'launch':str(D/'LAUNCH.json'),'prompt_bytes':F['prompt_bytes'],'print_timeout':'480s','outer_seconds':600}),flush=True)
stdinerror=None
try:
 p.stdin.write((json.dumps({'event':'user','message':{'content':(D/'PROMPT.txt').read_text()}},ensure_ascii=False)+'\n').encode());p.stdin.close()
except BrokenPipeError as e:stdinerror=type(e).__name__
with (D/'AVAILABLE-TOOL-TRANSPORT.jsonl').open('x') as transport:
 for raw in p.stdout:
  try:v=json.loads(raw)
  except Exception:errors+=1;continue
  if not isinstance(v,dict):errors+=1;continue
  event=v.get('event');counts[str(event)]+=1;marker(v.get('error'),'provider_error_event')
  if event=='init':
   init=v.get('init',{});conversation_ids.update([init['conversation_id']] if isinstance(init.get('conversation_id'),str) else []);models.append({k:init[k] for k in ['model','model_slug','model_name','effort','model_selection','permission_mode','mode','cwd','tools','conversation_id'] if k in init})
  elif event=='step_update':
   u=v.get('step_update',{});conversation_ids.update([u['conversation_id']] if isinstance(u.get('conversation_id'),str) else []);typ=u.get('step_type');types[str(typ)]+=1;marker(u.get('error'),'provider_step_error')
   if typ=='tool' or 'tool_info' in u:
    safe={k:u[k] for k in ['conversation_id','step_index','state','step_type','tool_name','duration_seconds','tool_info','error'] if k in u};tools.append(safe);transport.write(json.dumps(safe,ensure_ascii=False)+'\n');transport.flush()
    stop('FORBIDDEN_ANY_TOOL_ACTION','available_tool_metadata',json.dumps(safe,ensure_ascii=False))
  elif event=='result':
   result=v.get('result',{});conversation_ids.update([result['conversation_id']] if isinstance(result.get('conversation_id'),str) else []);marker(result.get('error'),'provider_terminal_error');terminal={k:result[k] for k in ['status','response','error','duration_seconds','num_turns','usage','conversation_id','model','model_slug','model_name','effort','model_selection'] if k in result}
code=p.wait();thread.join(timeout=2);report=(terminal or {}).get('response','');report=report if isinstance(report,str) else json.dumps(report,ensure_ascii=False);(D/'REPORT.md').write_text(report);post=pins();write('POSTCALL-PINS.json',post);errbytes=(D/'PROVIDER.stderr').read_bytes();r={'actual_exit':code,'start_utc':startutc,'end_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'elapsed_seconds':time.monotonic()-start,'requested_model':'gemini-4-argon-high','requested_effort':'high in pinnedmodelslug','returned_init_selection':models,'backendcanonical_weights_attestation':False,'event_counts':dict(counts),'step_type_counts':dict(types),'terminal':terminal,'available_tool_steps':tools,'stream_parse_errors':errors,'stdin_error':stdinerror,'report_bytes':len(report.encode()),'report_sha256':sha(report.encode()),'stderr_bytes':len(errbytes),'stderr_sha256':sha(errbytes),'stderr_encoding':'exactbinarycapture; decodeUTF8withreplacementonlyformarkers','stop_events':stops,'provider_print_timeout_seconds':480,'GNU_outer_deadline_seconds':600,'termination_grace_seconds':10,'no_autorestart_or_secondprovider_call':True,'all_input_pins_unchanged':post['all_match'],'tail_sentinel_present':'END-R148-NARROW-NONSECURITY-LITERAL-CORRECTION-PROPOSAL-NO-LAUNCH' in report,'private_thought_text_or_rawproviderstream_retained':False,'source_compiler_test_SDK_native_W_Git_actions_by_controller':'NONE; immutable sourcefile reads/paging/hashinspection, R-only outputs and oneproviderreview only','actual_role':'actual_prior_nonsecurity_owner_narrow_literal_clarification','no_security_vote_replacement':True,'required_pages':0,'requested_conversation_id':M['requested_conversation_id'],'observed_conversation_ids':sorted(conversation_ids),'actual_resumed_context_matches':bool(conversation_ids) and conversation_ids=={M['requested_conversation_id']},'no_new_full_source_review':True,'native_authority':0};write('RESULT.json',r);print(json.dumps({k:r[k] for k in ['actual_exit','elapsed_seconds','report_bytes','report_sha256','event_counts','step_type_counts','stop_events','all_input_pins_unchanged']}),flush=True)
