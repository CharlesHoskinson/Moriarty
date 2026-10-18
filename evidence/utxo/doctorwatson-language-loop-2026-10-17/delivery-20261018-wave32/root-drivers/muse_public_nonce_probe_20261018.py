from pathlib import Path
import datetime,hashlib,json,os,selectors,signal,subprocess,tempfile,time,shutil
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017');P=R/'concurrency32-wave-20261018/cli_sdk_boundary_sol/muse-protocol';D=R/'muse-public-nonce-probe-20261018';D.mkdir(mode=0o700)
work=Path(tempfile.mkdtemp(prefix='doctorwatson-muse-nonce-'))
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
write=lambda name,value:(D/name).write_text(json.dumps(value,indent=2)+'\n')
argv=['/usr/bin/timeout','--signal=TERM','--kill-after=5s','120s','/home/charl/.local/bin/muse','exec','--json','--provider','meta','--model','muse-wonder-1.0','--reasoning-effort','high','--preset','empty','--workspace',str(work),'--worktree','off','--disable-write','--disable-shell','--disable-web-tools','--disable-reminders','--disable-request-user-input','--no-foreign-personal-context','--no-session-log','--max-model-steps','1','--output-schema',str(P/'NONCE-OUTPUT-SCHEMA.json'),'--prompt-file',str(P/'NONCE-PROMPT.txt')]
write('START.json',{'startedAt':now(),'argv':argv,'promptSha256':sha(P/'NONCE-PROMPT.txt'),'schemaSha256':sha(P/'NONCE-OUTPUT-SCHEMA.json'),'scope':'One fresh public nonce format observation only. No review/admission/model-attestation.','capturePolicy':'new stdout memory+write-only private; allowlisted structure only exported, no private reread.'})
events=[];errors=0;buffer=b'';count=0;eof=False
allowed={'completed','failed','cancelled','SUCCESS','success','result','assistant','user','system','message','final','analysis','error','text','init','turn/completed','item/completed','muse-wonder-1.0','meta'}
def skeleton(value,depth=0):
 if isinstance(value,dict):return {k:skeleton(v,depth+1) for k,v in value.items()} if depth<5 else {'objectKeys':list(value)}
 if isinstance(value,list):return {'arrayLength':len(value),'firstShapes':[skeleton(v,depth+1) for v in value[:3]]} if depth<5 else {'arrayLength':len(value)}
 if isinstance(value,str):
  if value in allowed or value in {'muse-public-format-nonce-20261018-01','format-nonce-only'}:return {'type':'string','allowlistedLiteral':value}
  return {'type':'string','utf8Bytes':len(value.encode()),'redacted':True}
 if value is None:return {'type':'null'}
 return {'type':type(value).__name__,'redacted':True}
def line(raw):
 global errors
 if not raw.strip():return
 try:events.append(json.loads(raw.decode('utf-8')))
 except (ValueError,UnicodeError):errors+=1
start=time.monotonic();deadline=start+125
try:
 with (D/'provider.private.jsonl').open('xb') as private_out,(D/'stderr.private').open('xb') as private_err:
  child=subprocess.Popen(argv,cwd=work,stdin=subprocess.DEVNULL,stdout=subprocess.PIPE,stderr=private_err,start_new_session=True)
  write('CHILD.json',{'pid':child.pid,'ownedProcessGroup':child.pid})
  try:
   with selectors.DefaultSelector() as sel:
    sel.register(child.stdout,selectors.EVENT_READ)
    while time.monotonic()<deadline:
     if not sel.select(min(.2,max(0,deadline-time.monotonic()))):continue
     chunk=os.read(child.stdout.fileno(),65536)
     if not chunk:eof=True;break
     private_out.write(chunk);private_out.flush();count+=len(chunk);buffer+=chunk
     while b'\n' in buffer:
      raw,buffer=buffer.split(b'\n',1);line(raw)
   if eof and buffer:line(buffer)
   try:child.wait(timeout=max(.01,deadline-time.monotonic()))
   except subprocess.TimeoutExpired:os.killpg(child.pid,signal.SIGKILL);child.wait(timeout=1)
  finally:
   child.stdout.close()
   try:os.killpg(child.pid,signal.SIGKILL)
   except ProcessLookupError:pass
   if child.poll() is None:child.wait(timeout=1)
 remaining=[int(pid) for pid,sid in (row.split() for row in subprocess.check_output(['ps','-eo','pid=,sid=']).decode().splitlines()) if int(sid)==child.pid]
 receipt={'endedAt':now(),'exitCode':child.returncode,'wallSeconds':time.monotonic()-start,'stdoutBytes':count,'captureReachedEOF':eof,'parsedValues':len(events),'parseErrors':errors,'ownedSessionRemaining':remaining,'eventStructuralSkeletons':[skeleton(v) for v in events],'terminalReviewSelected':False,'actualBackendIdentityAttested':False,'publicNonceOnly':True}
 write('FORMAT-RECEIPT.json',receipt);print(json.dumps({k:v for k,v in receipt.items() if k!='eventStructuralSkeletons'}),flush=True)
finally:
 shutil.rmtree(work);write('CLEANUP.json',{'recordedAt':now(),'ownedRoot':str(work),'existsAfter':work.exists()})
