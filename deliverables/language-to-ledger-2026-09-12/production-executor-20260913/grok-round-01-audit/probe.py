import sys,json,pathlib,copy,time,hashlib,traceback,fcntl,os
ROOT=pathlib.Path('/home/charl/Moriarty-wt-moriarty-release-afk-20260913-implement-production-executor')
sys.path.insert(0,str(ROOT/'plugins/moriarty-dev/tests/loan_executor'))
from harness import LoanFixture,ExecutorRun,write_json,IMM
from moriarty_dev import runner,accounting,loan_executor
from moriarty_dev.loan_exit_retention.exit_retention import run_bounded
OUT=pathlib.Path('/tmp/moriarty-production-grok-r1-independent-probes'); rows=[]
def emit(name,**v):
 rows.append(dict(name=name,**v));(OUT/'results.json').write_text(json.dumps(rows,indent=2));print(json.dumps(rows[-1]),flush=True)
def read(fx,r,doc):
 write_json(fx.loaded_plan.result_path,doc)
 try:
  got,problem=runner._read_result_document(fx.root,fx.loaded_plan,fx.runner_digest,fx.reservation_id,r.digest,fx.runner)
  return {'accepted':got is not None,'problem':problem}
 except Exception as e:return {'exception':type(e).__name__,'message':str(e)}
fx=LoanFixture(root=OUT/'normal');fx.root.mkdir();fx.build();r=ExecutorRun(fx);r.run();doc=copy.deepcopy(r.result)
emit('normal_control',root=str(fx.root),external=str(fx.external),status=doc['status'],exit=r.outcome['exitCode'],**read(fx,r,doc))
# Retain exact normal evidence and mutate one artifact at a time, rehashing the declared reference.
for kind,change in [
 ('terminal-observation',lambda p:p.update(observed={'InvocationID':doc['invocationId'],'LoadState':'not-found','ExecMainStatus':'9'},returnCode=1,timedOut=True)),
 ('containment',lambda p:p.update(unit={'contained':False},prover={'contained':False},commands=[])),
 ('unit-startup',lambda p:p['observed'].update(LoadState='not-found',MainPID='0',Type='forking')),
 ('runtime-plan-comparison',lambda p:p.update(changedFields=['wallet.seedFile'],runtimeSha256='0'*64)),
 ('timer-armed',lambda p:p.update(active=False,problems=['not-active'])),
 ('explicit-stop',lambda p:p.update(returnCode=False)),
]:
 d=copy.deepcopy(doc);item=next(e for e in d['evidence'] if e['kind']==kind);p=fx.root/item['path'];raw=p.read_bytes();payload=json.loads(raw);change(payload);item['sha256']=write_json(p,payload)
 if kind=='terminal-observation':
  stopitem=next(e for e in d['evidence'] if e['kind']=='explicit-stop');sp=fx.root/stopitem['path'];sr=sp.read_bytes();sv=json.loads(sr);sv['terminalBeforeStopSha256']=item['sha256'];stopitem['sha256']=write_json(sp,sv)
 emit('result_inconsistent_'+kind,**read(fx,r,d));p.write_bytes(raw)
 if kind=='terminal-observation':sp.write_bytes(sr)
d=copy.deepcopy(doc);d.update(status='REFUSED',invocationId=None,rawMainExit={'kind':'unknown','code':None},failureCode='AUTHORITY_INVALID',evidence=[],terminalEvidencePersisted=False,stopReceiptPersisted=False,timerCancelReceiptPersisted=False,stopReturnCode=None,timerCancelReturnCode=None)
write_json(fx.loaded_plan.result_path,d);loaded,problem=runner._read_result_document(fx.root,fx.loaded_plan,fx.runner_digest,fx.reservation_id,r.digest,fx.runner)
emit('authenticated_refusal_without_no_start_evidence',accepted=loaded is not None,problem=problem,disposition=runner._disposition_for(2,loaded,True,False,False,True,progress=[],released=True))
chain=accounting.authenticate_authority(fx.root,fx.loaded_plan,fx.loaded_admission);hist=accounting.verify_history(chain)
def verify_obs():
 v=accounting.verify_observation(chain,hist,int(time.time()*1000),fx.loaded_plan.financial)
 return {'accepted':not isinstance(v,accounting.Refusal),'problem':repr(v) if isinstance(v,accounting.Refusal) else None}
p=fx.root/chain['context']['observerExecution']['sdkClosure']['path'];raw=p.read_bytes();p.unlink();emit('missing_sdk_closure',**verify_obs());p.write_bytes(raw)
emit('baseline_incomplete_observer_evidence',files=chain['context']['observerExecution']['files'],parentEvidence=json.loads(fx.parent_evidence_path.read_text()),**verify_obs())
# The current master has no preflight debit and no immutable grant marker/debit intent counterpart.
emit('missing_preflight_debit_and_application_marker',masterPurposes=[x.get('purpose') for x in json.loads(fx.master_path.read_text())['externalPackageCharges']],markerExists=(fx.root/pathlib.Path(fx.correspondence_path).parent/('grant-application-'+fx.projection_sha+'.json')).exists(),accepted=isinstance(accounting.validate_executor_context(fx.loaded_plan,fx.loaded_admission,r.payload,root=fx.root),accounting.CurrentExecution))
for label,change in [('wrong_first_role',lambda p:p['roles'].update(firstAddress='e'*64)),('overlap_directories',lambda p:p.update(outputDirectory=p['wallet']['stateDirectory'])),('uint128_overflow',lambda p:p['limits'].update(dustFee=str(2**128)))]:
 p=copy.deepcopy(fx.loaded_plan.financial);change(p);v=accounting.validate_financial_plan(p,chain['context']['expected'],int(time.time()*1000));emit(label,accepted=type(v)is dict,result=v)
# deadline write after cleanup boundary: explicit fake monotonic; real exclusive write.
s=loan_executor.Session(fx.loaded_plan,fx.loaded_admission,{'root':fx.root,'monotonic':lambda:1741.,'write_exclusive':loan_executor.production_write_exclusive});s.invocation=dict(r.payload,outerDeadlineMonotonic=1800*10**9);s.cuts={'cleanup':1740*10**9};s.status='PROCESS_UNKNOWN';s.failure_code='DEADLINE_EXCEEDED';s.plan.result_path=OUT/'late-result.json';document,out=loan_executor._write_result(s);emit('result_write_after_cleanup_cutoff',now=1741,cleanup=1740,persisted=out.persisted)
for n in (32,20000):
 out=run_bounded(lambda:'a'*n,0.15);emit('bounded_worker_result_'+str(n),status=out.status,elapsed=out.elapsed)
fx2=LoanFixture(root=OUT/'late-timer');fx2.root.mkdir();fx2.build();r2=ExecutorRun(fx2,script={'timerNextElapseOffsetSeconds':4});r2.run();timer=json.loads((fx2.evidence_dir/'timer-armed.json').read_text());emit('timer_four_seconds_late',root=str(fx2.root),external=str(fx2.external),status=r2.result['status'],exit=r2.outcome['exitCode'],timer=timer)
fx3=LoanFixture(root=OUT/'stop-failure');fx3.root.mkdir();fx3.build();r3=ExecutorRun(fx3,script={'stopReturnCode':1});r3.run();emit('actual_stop_failure_result_reader',root=str(fx3.root),external=str(fx3.external),status=r3.result['status'],failureCode=r3.result['failureCode'],rawMainExit=r3.result['rawMainExit'],**read(fx3,r3,r3.result))
