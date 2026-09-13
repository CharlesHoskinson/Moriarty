import sys,json,copy,hashlib
from pathlib import Path
wt=Path('/home/charl/Moriarty-wt-moriarty-release-afk-20260913-implement-production-executor')
sys.path.insert(0,str(wt/'plugins/moriarty-dev/tests/loan_executor'))
import harness
from moriarty_dev import loan_executor,runner,cli
fx=harness.LoanFixture().build()
rows=[]
try:
 plan=loan_executor.load_plan(fx.root,fx.plan_rel,fx.plan_sha)
 document={key:None for key in loan_executor.RESULT_KEYS}
 document.update(schema=loan_executor.RESULT_SCHEMA,allocationId=plan['allocationId'],actionId=plan['actionId'],candidateHash=plan['candidateHash'],runnerDigest=fx.runner_digest,chargeId=fx.runner['chargeId'],reservationId='isolated-reservation',invocationSha256='1'*64,unit=plan['unit'],invocationId='2'*32,status='PROCESS_SUCCESS',rawMainExit={'kind':'exit','code':0},terminalEvidencePersisted=True,stopReturnCode=0,stopErrorClass=None,stopReceiptPersisted=True,containmentComplete=True,timerCancelReturnCode=0,timerCancelReceiptPersisted=True,failureCode=None,evidence=[],outstandingOwners=[],retryAllowed=False,financialAcceptance='pending')
 cases=[('success-with-no-evidence',{}),('success-with-string-flags',{'terminalEvidencePersisted':'false','stopReceiptPersisted':'false','containmentComplete':'false','timerCancelReceiptPersisted':'false'}),('success-with-stop-error',{'stopErrorClass':'TimeoutError'}),('success-with-boolean-return-codes',{'stopReturnCode':False,'timerCancelReturnCode':False}),('refused-with-null-identities-and-outstanding-owner',{'status':'REFUSED','runnerDigest':None,'chargeId':None,'reservationId':None,'invocationSha256':None,'rawMainExit':{'kind':'unknown','code':None},'containmentComplete':False,'outstandingOwners':[{'owner':'timer','resource':'fixture','reason':'unresolved'}],'failureCode':'AUTHORITY_INVALID'})]
 for label,patch in cases:
  doc=copy.deepcopy(document);doc.update(patch);plan.result_path.write_text(json.dumps(doc))
  loaded,problem=runner._read_result_document(fx.root,plan,fx.runner_digest,'isolated-reservation','1'*64,fx.runner)
  refused=doc['status']=='REFUSED';disposition=runner._disposition_for(2 if refused else 0,loaded,not refused,not refused,False,True)
  mapping=cli._loan_disposition({'runnerReceipt':{'loanResult':{'disposition':disposition}}})
  rows.append({'case':label,'readerAccepted':loaded is not None,'problem':problem,'disposition':disposition,'cliMapping':mapping})
 result={'scope':'Isolated actual result reader/disposition/CLI-mapping boundary; surrounding handshake/ack/appended arguments supplied as controlled premises, not a full CLI exploit or valid execution claim. No service or canonical state used.','sourceMapSha256':'ba2d3d134c85178655ec3391b7ea72e30465044c89413ccec091b69bedbb3165','cases':rows}
 Path('/tmp/moriarty-production-r2-root-result-probes.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
finally:fx.cleanup()
