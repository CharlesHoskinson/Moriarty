import sys,pathlib,json,os,time,sqlite3,subprocess,hashlib
from contextlib import closing
from unittest.mock import patch
ROOT=pathlib.Path('/home/charl/Moriarty-wt-moriarty-release-afk-20260913-implement-production-executor')
sys.path.insert(0,str(ROOT/'plugins/moriarty-dev/tests/loan_executor'))
import harness
from moriarty_dev import loan_executor as le,loan_cleanup as lc,accounting,runner
from moriarty_dev.loan_exit_retention import exit_retention as er
OUT=pathlib.Path('/tmp/moriarty-production-r2-independent-probes');OUT.mkdir(exist_ok=True)
rows=[]
def record(name,**kw):
 row={'name':name,**kw};rows.append(row);(OUT/'results.json').write_text(json.dumps(rows,indent=2));print(name,json.dumps({k:v for k,v in kw.items() if k not in ('commands','result','stdout','stderr','faultHits')})[:700],flush=True)
def cli_case(name,driver=None,setup=None):
 fx=harness.LoanFixture().build()
 try:
  if setup:setup(fx)
  if driver:
   path=fx.root/'probe-driver.py';path.write_text(driver(fx))
   fx.write_runner(argv=[fx.python,'probe-driver.py','--plan',fx.plan_rel,'--sha256',fx.plan_sha,'--admission',fx.admission_rel])
  cmd=[sys.executable,str(harness.SCRIPTS/'moriarty_dev/cli.py'),'--repo',str(fx.root),'run','--action',fx.action['id'],'--json']
  done=subprocess.run(cmd,capture_output=True,text=True,timeout=90)
  with closing(sqlite3.connect(fx.db_path)) as c:
   reservations=c.execute('select status from reservations').fetchall(); events=c.execute('select event_kind,payload_json from events order by id').fetchall()
  result_path=fx.root/fx.plan['resultPath']; result=json.loads(result_path.read_text()) if result_path.exists() else None
  record(name,returncode=done.returncode,stdout=done.stdout,stderr=done.stderr,reservations=reservations,ownership=json.loads(fx.ownership_path.read_text())['state'],commands=fx.services.calls(),result=result,events=[k for k,v in events])
  (OUT/(name+'.json')).write_text(json.dumps({'stdout':done.stdout,'stderr':done.stderr,'result':result,'events':events,'commands':fx.services.calls()},indent=2))
 finally:fx.cleanup()
COMMON='''import sys,json,os
from pathlib import Path
sys.path.insert(0,%r)
from moriarty_dev import loan_executor as le
root=Path.cwd()
plan=le.load_plan(root,sys.argv[sys.argv.index('--plan')+1],sys.argv[sys.argv.index('--sha256')+1])
admission=le.load_admission(root,sys.argv[sys.argv.index('--admission')+1],plan)
'''
def forge(fx,status='PROCESS_SUCCESS',handshake=True):
 text=COMMON%str(harness.SCRIPTS)
 if handshake:text+='''channel=le.connect_parent_channel(admission,os.environ,le.production_boot_id,__import__('time').monotonic_ns)
assert not isinstance(channel,le.Refusal),channel
p=channel.invocation
'''
 else:text+='p={}\n'
 text+='''d={k:None for k in le.RESULT_KEYS}
d.update(schema=le.RESULT_SCHEMA,allocationId=plan['allocationId'],actionId=plan['actionId'],candidateHash=plan['candidateHash'],unit=plan['unit'],runnerDigest=p.get('runnerDigest'),chargeId=p.get('chargeId'),reservationId=p.get('reservationId'),invocationSha256=channel.invocation_sha256 if p else None,invocationId=None,status=%r,rawMainExit={'kind':'exit','code':0},terminalEvidencePersisted=True,stopReturnCode=0,stopReceiptPersisted=True,containmentComplete=True,timerCancelReturnCode=0,timerCancelReceiptPersisted=True,evidence=[],outstandingOwners=[],retryAllowed=False,financialAcceptance='pending')
'''%status
 if status=='REFUSED':text+="d.update(containmentComplete=False,outstandingOwners=[{'owner':'timer','resource':'unit','reason':'unknown'}],rawMainExit={'kind':'unknown','code':None})\n"
 text+="w=le.production_write_exclusive(plan.result_path,json.dumps(d).encode());assert w.persisted\n"
 if handshake:text+="channel.send('contained');channel.close()\n"
 text+='raise SystemExit(%d)\n'%(0 if status=='PROCESS_SUCCESS' else 2)
 return text
cli_case('normal_cli')
cli_case('forged_exact_authenticated_empty_evidence',lambda fx:forge(fx))
cli_case('forged_early_refused_uncontained',lambda fx:forge(fx,'REFUSED',False))
def release_driver(fx):
 text=COMMON%str(harness.SCRIPTS)
 text+="deps=le.production_dependencies(root,service_table=%r)\n"%fx.service_table
 text+="ret=le.main(sys.argv[1:],dependencies=deps)\np=Path(%r);d=json.loads(p.read_text());d['runtimeReservationId']='different-owner';p.write_text(json.dumps(d));raise SystemExit(ret)\n"%str(fx.ownership_path)
 return text
cli_case('ownership_release_refused_after_success',release_driver)
# Authentic ordinary CLI with invalid mutable funding evidence, still bound through correspondence.
for name,patches in [('zero_wrong_token_units',{'amounts':{'WRONG':'0'},'unitNames':{'WRONG':'wrong'}}),('wrong_observer_source',{'sourceSha256':'0'*64}),('pending_use_substituted_receipt',{'pendingUseDispositionSha256':None})]:
 def setup(fx,patches=patches):
  patches=dict(patches)
  if patches.get('pendingUseDispositionSha256','missing') is None:patches['pendingUseDispositionSha256']=harness.sha256_file(fx.root/'deliverables/loan-fixture/immutable/loan-attempt-1.json')
  fx.write_observation(**patches);fx.write_correspondence()
 cli_case('accounting_'+name,setup=setup)
# Faults reaching the actual writer used by the final collector and caller.
for target in ('runtime-plan-comparison.json','timer-armed.json','terminal-observation.json','explicit-stop.json','containment-normal-cleanup.json','loan-process-result.json'):
 for fault in ('write_error','file_error','dir_error','dir_stall'):
  fx=harness.LoanFixture().build(); run=harness.ExecutorRun(fx); marker=OUT/(target+'-'+fault+'.hits')
  original=le._write_exclusive_bytes; original_fsync=os.fsync; original_write=os.write
  def actual(path,data,mode):
   if pathlib.Path(path).name!=target:return original(path,data,mode)
   def hit(fd,call):
    p=os.readlink('/proc/self/fd/'+str(fd));isdir=pathlib.Path(p).is_dir()
    applies=(fault=='write_error' and call=='write') or (call=='fsync' and ((fault=='file_error' and not isdir) or (fault.startswith('dir_') and isdir)))
    if applies:
     with marker.open('a') as f:f.write(json.dumps({'pid':os.getpid(),'fdPath':p,'call':call,'fault':fault})+'\n')
     if fault=='dir_stall':time.sleep(3)
     raise OSError('independent actual '+fault)
   def wr(fd,raw):hit(fd,'write');return original_write(fd,raw)
   def sy(fd):hit(fd,'fsync');return original_fsync(fd)
   with patch.object(os,'write',wr),patch.object(os,'fsync',sy):return original(path,data,mode)
  orig_writer=le.production_write_exclusive
  def writer(path,data,mode=0o600,timeout_seconds=25):return orig_writer(path,data,mode,0.2 if pathlib.Path(path).name==target and fault=='dir_stall' else timeout_seconds)
  run.writer=writer
  try:
   start=time.monotonic()
   with patch.object(le,'_write_exclusive_bytes',actual):run.run()
   hits=[json.loads(s) for s in marker.read_text().splitlines()] if marker.exists() else []
   record('persistence_'+target+'_'+fault,exitCode=run.outcome['exitCode'],status=run.result['status'],failureCode=run.result['failureCode'],elapsed=time.monotonic()-start,hitCount=len(hits),faultHits=hits,workersAlive=[h['pid'] for h in hits if pathlib.Path('/proc/'+str(h['pid'])).exists()],result=run.result,commands=run.calls())
  finally:fx.cleanup()
# Clock advances during setup, without resetting the authenticated outer start.
fx=harness.LoanFixture().build();run=harness.ExecutorRun(fx);deps=run.dependencies();clock=[0.0];base=deps['run_command'];wall=deps['wall_time_ms'];mono=deps['monotonic']
def age_transport(argv,timeout):
 result=base(argv,timeout)
 if '--format' in argv and '{{json .}}' in argv:clock[0]=110.0
 return result
deps['run_command']=age_transport;deps['wall_time_ms']=lambda:wall()+int(clock[0]*1000);deps['monotonic']=lambda:mono()+clock[0]
# Start observation at age20, then push to130 during prestart (within outer setup120).
orig=accounting.verify_observation
observation=fx.root/fx.receipt_path;v=json.loads(observation.read_text());stamp=harness.iso(harness.utc_now()-harness.datetime.timedelta(seconds=20));v['observedAt']=stamp;v['syncTime']=stamp
# Updating current observation/correspondence requires new payload digest, so use injected base wall+20 instead.
deps['wall_time_ms']=lambda:wall()+20000+int(clock[0]*1000)
try:
 out=le.execute_once(fx.loaded_plan,fx.loaded_admission,deps);record('observation_ages_past120_during_setup',exitCode=out['exitCode'],status=out['result']['status'],commands=run.calls(),virtualAgeAtStartSeconds=130)
finally:fx.cleanup()
# Exact running-classification contradictions and cleanup replacement/budget tests.
from moriarty_dev.loan_exit_retention.loan_exit_operator import retain_loan_main_exit
for key,value in [('Type','oneshot'),('RemainAfterExit','no'),('Transient','no'),('ExecMainCode','1')]:
 fx=harness.LoanFixture().build()
 startup=dict(harness.fake_services.STARTUP)
 fields=dict(startup,Type='exec',RemainAfterExit='yes',Transient='yes',Result='success',ExecMainCode='0',ExecMainStatus='0');fields[key]=value
 class T:
  def show(self,unit,properties,seconds):return er.CommandResult(['show'],0,'\n'.join(k+'='+v for k,v in fields.items()))
  def stop(self,*args):raise AssertionError('must not stop')
 directory=fx.evidence_dir/'collector';directory.mkdir()
 try:
  res=retain_loan_main_exit(harness.UNIT,startup,T(),directory,show_seconds=5,stop_seconds=25,persist_timeout_seconds=1)
  record('running_contradiction_'+key,status=res['status'],classification=res.get('classification'),files=[p.name for p in directory.iterdir()])
 finally:fx.cleanup()
