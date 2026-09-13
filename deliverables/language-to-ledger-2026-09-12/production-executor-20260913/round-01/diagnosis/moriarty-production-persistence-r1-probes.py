import sys,pathlib,json,os,time,contextlib,hashlib
from unittest.mock import patch
ROOT=pathlib.Path('/tmp/moriarty-production-persistence-diagnosis-r1-work')
sys.path.insert(0,str(ROOT/'plugins/moriarty-dev/tests/loan_executor'))
import harness
from moriarty_dev import loan_executor,loan_cleanup
from moriarty_dev.loan_exit_retention import exit_retention as er
import test_loan_cleanup as tc
OUT=pathlib.Path('/tmp/moriarty-production-persistence-r1-probes');OUT.mkdir(exist_ok=True)
rows=[]
def execute_case(name, writer=None, target=None, fault=None):
 fx=harness.LoanFixture().build()
 run=harness.ExecutorRun(fx,writer=writer)
 marker=OUT/(name+'.hits')
 orig_sync=er._write_exclusive_sync
 orig_fsync=os.fsync
 orig_save=er.durable_exclusive_save
 def sync(path,raw):
  if pathlib.Path(path).name != target:return orig_sync(path,raw)
  def fsync(fd):
   link=os.readlink('/proc/self/fd/'+str(fd))
   isdir=pathlib.Path(link).is_dir()
   hit= ('dir' in fault and isdir) or ('dir' not in fault and not isdir)
   if hit:
    with marker.open('a') as f:f.write(json.dumps({'pid':os.getpid(),'fdPath':link,'fault':fault})+'\n')
    if 'stall' in fault:time.sleep(5)
    raise OSError('probe real fsync failure')
   return orig_fsync(fd)
  with patch.object(os,'fsync',fsync):return orig_sync(path,raw)
 def save(path,value,timeout_seconds=None):return orig_save(path,value,min(timeout_seconds,0.2) if pathlib.Path(path).name==target else timeout_seconds)
 try:
  started=time.monotonic()
  with patch.object(er,'_write_exclusive_sync',sync),patch.object(er,'durable_exclusive_save',save):run.run()
  row={'name':name,'elapsed':time.monotonic()-started,'exitCode':run.outcome['exitCode'],'result':run.result,'writerCalls':run.writer.calls,'commands':run.calls(),'faultHits':marker.read_text().splitlines() if marker.exists() else []}
  if target:
   p=fx.evidence_dir/'collector'/target
   row['targetFileExists']=p.exists()
   row['targetFileSha256']=hashlib.sha256(p.read_bytes()).hexdigest() if p.exists() else None
  rows.append(row);print(name,row['exitCode'],run.result['status'],run.result['failureCode'],len(row['faultHits']),flush=True)
 finally:fx.cleanup()
execute_case('baseline')
for name in ('terminal-observation.json','explicit-stop.json'):
 execute_case('missed_fail_'+name,writer=harness.FaultyWriter(fail=[name]))
 execute_case('missed_timeout_'+name,writer=harness.FaultyWriter(timeout=[name]))
 for fault in ('file_error','dir_error','file_stall','dir_stall'):
  execute_case(name+'_'+fault,target=name,fault=fault)
# Probe actual cleanup transport boundary while retaining exact caller and fake underlying services.
for kind in ('original_showTimeout','actual_identity_timeout','actual_identity_nonzero','actual_identity_exception','after_kill_timeout'):
 case=tc.CleanupIdentity();case.setUp();case.services._save('script.json',{'showTimeout':True})
 base=case.run;hits=[]
 def transport(argv,seconds):
  identity='show' in argv and any(a=='--property=LoadState,ActiveState,SubState,MainPID,ControlGroup,InvocationID' for a in argv)
  if identity and kind!='original_showTimeout':
   hits.append(list(argv))
   if kind=='after_kill_timeout' and len(hits)==1:return base(argv,seconds)
   if kind=='actual_identity_exception':raise TimeoutError('probe observation unavailable')
   return loan_cleanup.CommandResult(argv,1 if kind=='actual_identity_nonzero' else None,'','probe',timed_out=kind!='actual_identity_nonzero')
  return base(argv,seconds)
 facts=loan_cleanup.contain_owned(case.intent,tc.prover_receipt('2026-09-13T20:00:00Z'),tc.unit_receipt('f'*32),transport,25)
 rows.append({'name':kind,'facts':facts,'injectionCalls':hits})
 print(kind,facts['unit']['contained'],facts['unit']['disposition'],len(hits),flush=True)
(OUT/'results.json').write_text(json.dumps(rows,indent=2)+'\n')
