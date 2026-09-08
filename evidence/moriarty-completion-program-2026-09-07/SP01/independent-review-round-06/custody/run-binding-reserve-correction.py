from pathlib import Path
import sys,os,json,subprocess,time,datetime,hashlib,selectors,signal
S=Path(__file__).resolve().parent
B=json.loads((S/'binding.json').read_text());W=Path(B['worktree']);kind=sys.argv[1]
limits=B['resources']['verificationCommands']
if kind=='binding-reserve-correction-worker':
 cap=1680;prompt=S/'binding-reserve-correction-prompt.md'
 shell='source /home/charl/.agents/skills/foreman/scripts/adapters/grok.sh\nadapter_implement_argv grok "$1" "$2"\nprintf "%s\\0" "${ADAPTER_ARGV[@]}"'
 raw=subprocess.check_output(['/bin/bash','-c',shell,'adapter',str(prompt),str(W)])
 command=[x.decode() for x in raw.split(b'\0') if x and x!=b'--no-leader']
 command[0]='/home/charl/.local/bin/grok';command[command.index('--output-format')+1]='json'
 command+=['--reasoning-effort','high','--no-subagents','--disable-web-search','--max-turns','48']
else:
 cap=limits[kind];command=['/usr/bin/python3',str(S/(kind+'.py'))]
ledger=json.loads((S/'ledger.json').read_text());assert not ledger['stopped'],'Campaign stopped. Record amendment before continuing.'
assert not any(x['id']==kind for x in ledger['charges']),'No implicit rerun'
ledger['charges'].append({'id':kind,'seconds':cap,'at':datetime.datetime.now(datetime.timezone.utc).isoformat()})
if kind.endswith('worker'):ledger['workerDispatches']+=1;assert ledger['workerDispatches']<=B['resources']['workerDispatches']
assert sum(x['seconds'] for x in ledger['charges'])<=B['resources']['allocationSeconds']
(S/'ledger.json').write_text(json.dumps(ledger,indent=2)+'\n')
old=Path('/home/charl/.local/state/moriarty/mc01-supervised-20260907/budget.json');d=json.loads(old.read_text());d['externalPackageCharges'].append({'package':'MC02','id':'sp05-kernel-custody-'+kind,'seconds':cap,'basis':'Full bound before dispatch, including termination grace'})
if kind.endswith('worker'):d['externalWorkerDispatches']+=1
old.write_text(json.dumps(d,indent=2)+'\n')
# Mount tmpfs inside this process tree only. Native process limits cover all children.
mount_script='set -eu\n/usr/bin/mount -t tmpfs -o size=128m,mode=1777 tmpfs /tmp\nexec "$@"\n'
unit='moriarty-sp05-kernel-custody-'+kind+'-'+str(os.getpid())
cmd=['/usr/bin/systemd-run','--user','--quiet','--wait','--pipe','--unit='+unit,'-p','MemoryMax=2G','-p','MemorySwapMax=0','-p','CPUQuota=200%','-p','RuntimeMaxSec='+str(cap-5),'-p','TimeoutStopSec=2','-p','KillMode=control-group','-p','OOMPolicy=kill','--working-directory='+str(W),'--setenv=PATH='+os.environ['PATH'],'--setenv=MORIARTY_RUNTIME_NODE_MODULES='+B['runtimeNodeModules'],'--setenv=NODE_OPTIONS=--max-old-space-size=512','/usr/local/bin/node','/home/charl/foreman/skills/foreman/runtime/dist/foreman-launch.js','--timeout',str(cap-10),'--grace','2','--require-containment','strong','--capability-file',str(S/(kind+'-capability.json')),'--heartbeat-file',str(S/(kind+'-heartbeat.jsonl')),'--','/usr/bin/unshare','--user','--map-root-user','--mount','/bin/sh','-c',mount_script,'moriarty-tmpfs',*command]
head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=W,text=True).strip();status=subprocess.check_output(['git','status','--porcelain'],cwd=W)
(S/(kind+'-command.json')).write_text(json.dumps({'argv':cmd,'cwd':str(W),'beforeHead':head,'beforeStatusSha256':hashlib.sha256(status).hexdigest()},indent=2)+'\n')
start=time.monotonic();p=subprocess.Popen(cmd,stdin=subprocess.DEVNULL,stdout=subprocess.PIPE,stderr=subprocess.PIPE,start_new_session=True)
sel=selectors.DefaultSelector();outputs={};total=sum(f.stat().st_size for f in S.iterdir() if f.is_file());storage_stop=False;properties=None
for stream,name in [(p.stdout,'stdout'),(p.stderr,'stderr')]:sel.register(stream,selectors.EVENT_READ,name);outputs[name]=open(S/(kind+'.'+name),'wb')
while sel.get_map():
 for key,_ in sel.select(.2):
  data=os.read(key.fileobj.fileno(),65536)
  if not data:sel.unregister(key.fileobj);continue
  total+=len(data)
  buildTotal=sum(f.stat().st_size for f in (S/'build').rglob('*') if f.is_file() and not f.is_symlink()) if (S/'build').exists() else 0
  if total>B['resources']['retainedBytes'] or buildTotal>B['resources']['buildBytes']:
   storage_stop=True;subprocess.run(['systemctl','--user','stop',unit],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL);continue
  outputs[key.data].write(data);outputs[key.data].flush()
 if properties is None and time.monotonic()-start>.1:
  q=subprocess.run(['systemctl','--user','show',unit,'-p','MemoryMax','-p','MemorySwapMax','-p','CPUQuotaPerSecUSec','-p','KillMode','-p','RuntimeMaxUSec','-p','ControlGroup'],capture_output=True,text=True)
  if q.returncode==0 and 'MemoryMax=2147483648' in q.stdout:properties=q.stdout
 if time.monotonic()-start>cap:
  subprocess.run(['systemctl','--user','stop',unit],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
  if p.poll() is None:os.killpg(p.pid,signal.SIGKILL)
rc=p.wait()
for f in outputs.values():f.close()
endhead=subprocess.check_output(['git','rev-parse','HEAD'],cwd=W,text=True).strip();endstatus=subprocess.check_output(['git','status','--porcelain'],cwd=W)
receipt={'kind':kind,'exitCode':rc,'elapsedSeconds':time.monotonic()-start,'chargedSeconds':cap,'beforeHead':head,'afterHead':endhead,'beforeStatusSha256':hashlib.sha256(status).hexdigest(),'afterStatusSha256':hashlib.sha256(endstatus).hexdigest(),'unauthorizedGitActivity':head!=endhead,'storageStop':storage_stop,'effectiveProperties':properties,'scope':B['scope']}
if kind.endswith('worker'):
 try:
  j=json.load(open(S/(kind+'.stdout')));receipt['modelUsage']=j.get('modelUsage');receipt['resultText']=j.get('text');receipt['stopReason']=j.get('stopReason')
 except Exception as e:receipt['resultParseError']=str(e)
receipt['status']=endstatus.decode()
(S/(kind+'-receipt.json')).write_text(json.dumps(receipt,indent=2)+'\n')
# RED nonzero is expected and inspected by parent. All other failure stops require amendment.
if (rc!=0 and kind!='red') or storage_stop or head!=endhead:
 ledger=json.loads((S/'ledger.json').read_text());ledger['stopped']=True;ledger['reason']=kind+' failed';(S/'ledger.json').write_text(json.dumps(ledger,indent=2)+'\n')
print(json.dumps(receipt,indent=2))
sys.exit(rc)
