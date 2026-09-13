exec(open('/tmp/moriarty-production-r2-audit-probes.py').read().split("cli_case('normal_cli')")[0])
rows=[]
def record(name,**kw):
 row={'name':name,**kw};rows.append(row);(OUT/'supplement-results.json').write_text(json.dumps(rows,indent=2));print(name,json.dumps({k:v for k,v in kw.items() if k not in ('commands','result','stdout','stderr','faultHits','facts')})[:800],flush=True)
from moriarty_dev.loan_exit_retention.loan_exit_operator import retain_loan_main_exit
startup={'LoadState':'loaded','ActiveState':'active','SubState':'running','MainPID':'4242','ControlGroup':'/x','ActiveEnterTimestampMonotonic':'100','InvocationID':'f'*32}
for key,value in [('Type','oneshot'),('RemainAfterExit','no'),('Transient','no'),('ExecMainCode','1')]:
 fx=harness.LoanFixture().build()
 fields=dict(startup,Type='exec',RemainAfterExit='yes',Transient='yes',Result='success',ExecMainCode='0',ExecMainStatus='0');fields[key]=value
 class T:
  def show(self,unit,properties,seconds):return er.CommandResult(['show'],0,'\n'.join(k+'='+v for k,v in fields.items()))
  def stop(self,*args):raise AssertionError('must not stop')
 directory=fx.evidence_dir/'collector';directory.mkdir()
 try:
  res=retain_loan_main_exit(harness.UNIT,startup,T(),directory,show_seconds=5,stop_seconds=25,persist_timeout_seconds=1)
  record('running_contradiction_'+key,status=res['status'],classification=res.get('classification'),files=[p.name for p in directory.iterdir()])
 finally:fx.cleanup()
import test_loan_cleanup as tc
for mode in ('unit_replaced_between_kill_and_stop','different_container_receipt_id','different_unit_receipt_name','elapsed_budget'):
 case=tc.CleanupIdentity();case.setUp();base=case.run; elapsed=[0.0];command_times=[]
 pr=tc.prover_receipt('2026-09-13T20:00:00Z');ur=tc.unit_receipt('f'*32)
 if mode=='different_container_receipt_id':pr['containerId']='b'*64
 if mode=='different_unit_receipt_name':ur['unit']='other.service'
 def trans(argv,seconds):
  command_times.append({'start':elapsed[0],'seconds':seconds,'argv':argv});elapsed[0]+=0.9
  ret=base(argv,seconds)
  if mode=='unit_replaced_between_kill_and_stop' and 'systemctl' in ' '.join(argv) and 'kill' in argv:
   case.services.set_unit('fixture.service',dict(startup,InvocationID='a'*32))
  return ret
 facts=lc.contain_owned(case.intent,pr,ur,trans,1 if mode=='elapsed_budget' else 25)
 record(mode,complete=facts['complete'],elapsedSimulated=elapsed[0],commandTimes=command_times,facts=facts)
 __import__('shutil').rmtree(case.tmp)
# Bad container argv is admitted by actual execute_once; no real services.
for mode in ('arbitrary_prover_argv','missing_start_timestamp','timer_next_elapse_mismatch','late_result_success'):
 fx=harness.LoanFixture().build();run=harness.ExecutorRun(fx);deps=run.dependencies();base=deps['run_command'];clock=[0.0];mono=deps['monotonic']
 if mode=='arbitrary_prover_argv':
  cfg=fx.services._load('container.json',{});cfg['Args']=['/control/prover-control','--','/bin/sh','-c','unreviewed'];fx.services.seed_container(cfg)
 def trans(argv,seconds):
  ret=base(argv,seconds)
  if mode=='missing_start_timestamp' and 'show' in argv and 'ActiveEnterTimestampMonotonic' in argv[-1]:ret.stdout=ret.stdout.replace('ActiveEnterTimestampMonotonic=', 'UnusedTimestamp=')
  if mode=='timer_next_elapse_mismatch' and 'show' in argv and 'NextElapseUSecMonotonic' in argv[-1] and 'LoadState=loaded' in ret.stdout:
   ret.stdout='LoadState=loaded\nActiveState=active\nSubState=waiting\nNextElapseUSecMonotonic=9999999999999999999\nUnit=unrelated.service\n'
  if mode=='late_result_success':
   if 'show' in argv and 'Result' in argv[-1]:clock[0]=1738.0
   elif clock[0]:clock[0]+=1
  return ret
 deps['run_command']=trans;deps['monotonic']=lambda:mono()+clock[0]
 def contain(ownership,deadline_ns,purpose='safety-cleanup'):
  remaining=max(1,int((deadline_ns-int(deps['monotonic']()*1e9))/1e9))
  return lc.contain_owned(ownership['intent'],ownership.get('proverReceipt'),ownership.get('unitReceipt'),trans,remaining,purpose,unit_started=ownership.get('unitStarted'),prover_started=ownership.get('proverStarted'))
 deps['observe_containment']=contain
 try:
  out=le.execute_once(fx.loaded_plan,fx.loaded_admission,deps);record(mode,exitCode=out['exitCode'],status=out['result']['status'],virtualOffsetSeconds=clock[0],result=out['result'],commands=run.calls())
 finally:fx.cleanup()
# Current budget lock must serialize master replacement but runner never opens it.
held=[]
def lock_setup(fx):
 import fcntl
 fd=os.open(fx.master_path.with_suffix('.lock'),os.O_CREAT|os.O_RDWR,0o600);fcntl.flock(fd,fcntl.LOCK_EX|fcntl.LOCK_NB);held.append(fd)
cli_case('master_lock_held_during_handover',setup=lock_setup)
for fd in held:os.close(fd)
# Real malformed result file type blocks the actual CLI after child exit.
fx=harness.LoanFixture().build()
try:
 driver=fx.root/'fifo-driver.py';driver.write_text("import json,sys,os\nfrom pathlib import Path\np=json.loads(Path(sys.argv[sys.argv.index('--plan')+1]).read_text());os.mkfifo(p['resultPath'])\n")
 fx.write_runner(argv=[fx.python,'fifo-driver.py','--plan',fx.plan_rel,'--sha256',fx.plan_sha,'--admission',fx.admission_rel])
 proc=subprocess.Popen([sys.executable,str(harness.SCRIPTS/'moriarty_dev/cli.py'),'--repo',str(fx.root),'run','--action',fx.action['id'],'--json'],stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
 try:out,err=proc.communicate(timeout=6);timed=False
 except subprocess.TimeoutExpired:timed=True;wchan=pathlib.Path('/proc/'+str(proc.pid)+'/wchan').read_text();proc.kill();out,err=proc.communicate()
 record('fifo_result_blocks_actual_cli',timedOut=timed,waitChannel=wchan if timed else None,returnCode=proc.returncode)
finally:fx.cleanup()
# Required live labels are rejected by the actual shared binding validator.
fx=harness.LoanFixture().build()
try:
 from moriarty_dev import records
 campaign=fx.campaigns['campaigns']['sp01-loan-swap-grok-01'];binding=fx.root/campaign['binding'];b=json.loads(binding.read_text());b['status']='admitted-i2-source-only';campaign['bindingSha256']=harness.write_json(binding,b);missing=[]
 ok,_=records._verify_binding(fx.root,'sp01-loan-swap-grok-01',campaign,missing)
 record('exact_live_binding_status',accepted=ok,missing=missing)
finally:fx.cleanup()
