"""Complete invoice public JSON/hash fidelity. No subject code import/evaluation."""
from pathlib import Path
import json,hashlib,collections
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017');D=Path(__file__).parent;Q=R/'invoice-receipt-normal90-preparation-20261018';C=R/'invoice-receipt-normal90-check-cycle-20261018'
def sha(b):return hashlib.sha256(b).hexdigest()
def load(p):return json.loads(p.read_bytes())
def write(p,v):p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(v,indent=2,ensure_ascii=False)+'\n')
def eq(a,b):return json.dumps(a,sort_keys=True,separators=(',',':'),ensure_ascii=False)==json.dumps(b,sort_keys=True,separators=(',',':'),ensure_ascii=False)
def ptr(v,p):
 for k in p.split('/')[1:] if p else []:v=v[int(k)] if isinstance(v,list) else v[k.replace('~1','/').replace('~0','~')]
 return v
def walk(v,p=''):
 if isinstance(v,dict) and v:
  for k,w in v.items():yield from walk(w,p+'/'+k.replace('~','~0').replace('/','~1'))
 elif isinstance(v,list) and v:
  for i,w in enumerate(v):yield from walk(w,p+'/'+str(i))
 else:yield p,v
copies=[]
def copy(src,dst):
 b=src.read_bytes();dst.parent.mkdir(parents=True,exist_ok=True);dst.write_bytes(b);assert dst.read_bytes()==b;copies.append({'source':str(src),'copy':str(dst.relative_to(D)),'bytes':len(b),'sha256':sha(b),'byteIdentical':True})
for p in sorted(C.rglob('*')):
 if p.is_file():copy(p,D/'raw-actual'/p.relative_to(C))
assert len(copies)==16  # five captures plus eleven public controls/streams/rootdisposition
assert sha((C/'ROOT-ACTUAL-DISPOSITION.json').read_bytes())=='b2ce4ce50f6662ff08d79cc285190a982addc2993fc601c983a99d6ee112a4df'
assert sha((C/'RESULT.json').read_bytes())=='796f51f479c34544292fdf0a0c097748552edb5493698f7eb82fe4e8db69af22'
sources=load(Q/'SOURCE-PINS.json');assert len(sources['files'])==198
for row in sources['files']:
 p=Q/'current-source'/row['path'];assert len(p.read_bytes())==row['bytes'] and sha(p.read_bytes())==row['sha256'];copy(p,D/'source'/row['path'])
for n in ['BOUND-CANDIDATE.json','SOURCE-PINS.json','EXPECTATION-PINS.json','TOOL-PINS.json','EFFECT-CLOSURE.json','ROOT-SOURCE-BINDING.json','SOURCE-EXPOSURE.json','FIVE-CONSUMER-MAPPINGS.json','RUNTIME-REQUIREMENTS.json','ENVIRONMENT-DIFF.json','STATIC-INSPECTION.json','run_check.py','FREEZE.sha256','helper-vs-inspected-escrow.diff','CAPTURE-CONTRACT.json']:
 copy(Q/n,D/'check-preparation'/n)
parent=R/'pr232-pr234-next-consumer-plan-20261018'
for n in ['FULL-EIGHT-SCOPE-CUSTODY.json','PLAN.md','NEW-FIVE-CONSUMERS.json']:copy(parent/n,D/'parent-scope'/n)
binding=load(Q/'BOUND-CANDIDATE.json');assert sha((Q/'run_check.py').read_bytes())==binding['driverSha256'];controls=[]
for n,h in binding['controlHashes'].items():assert sha((Q/n).read_bytes())==h;controls.append({'name':n,'sha256':h,'verified':True})
expect=load(Q/'EXPECTATION-PINS.json');aliases=[]
for i,row in enumerate(expect['externalOriginalAndSourcePacketFiles']):
 p=Path(row['path']);assert p.stat().st_size==row['bytes'] and sha(p.read_bytes())==row['sha256'];copy(p,D/'original-external'/str(i).zfill(3)/p.name);aliases.append({'kind':'externalOriginal',**row,'verified':True})
assert len(expect['copiedIndependentFiles'])==18
for row in expect['copiedIndependentFiles']:
 assert (D/'source'/row['repositoryPath']).read_bytes()==Path(row['originalPath']).read_bytes();assert sha(Path(row['originalPath']).read_bytes())==row['sha256'];aliases.append({'kind':'original18Fixture',**row,'verified':True})
write(D/'ALIAS-AND-CONTROL-CHECKS.json',{'controls':controls,'aliases':aliases,'all198SourceCopiesMatchFrozenPins':True,'noProductExecution':True})
B=D/'source/crates/fixtures/invoice-receipt-resume';ER=load(B/'EXPECTED-RESUME.json');EJ=load(B/'EXPECTED-ALL-PREFIXES.json');W=load(B/'EXPECTED-LOGICAL-WORK.json');F=load(B/'EXPECTED-FINANCIAL-SUMMARY.json')
A={p.name:load(p)for p in (C/'invoice-captures').iterdir()};RS='resume-observations.json';JS='cross-segment-observations.json';SS='start-result.json';CS='reopened-continuation.json';ZS='resume-result.json'
resume=A[RS];journey=A[JS];waiting=A[SS]['Ok']['Waiting'];complete=A[ZS]['Ok']['Complete'];reopened=A[CS];st=waiting['segment'];su=complete['segment'];se=st['execution'];ue=su['execution']
assert list(A[SS])==['Ok'] and list(A[SS]['Ok'])==['Waiting'];assert list(A[ZS])==['Ok']and list(A[ZS]['Ok'])==['Complete']
result=load(C/'RESULT.json');terminal=load(C/'TERMINAL.json');captures=load(C/'INVOICE-CAPTURES.json');start=load(C/'START.json');plan=load(C/'CHECK-PLAN.json');proc=load(C/'PROCESS-CLEANUP.json');child=load(C/'CHILD.json');clean=load(C/'CLEANUP.json')
assert captures==result['invoiceCaptures']
for row in captures['files']:
 p=C/'invoice-captures'/row['name'];assert row['exists']and p.stat().st_size==row['bytes']and sha(p.read_bytes())==row['sha256']
assert sorted(A)==captures['directoryEntries'] and len(A)==5
for n,k in [('STDOUT.log','stdoutSha256'),('STDERR.log','stderrSha256')]:assert sha((C/n).read_bytes())==result[k]
assert plan['bindingSha256']==sha((Q/'BOUND-CANDIDATE.json').read_bytes())
assert plan['argv']==start['argv']and plan['environment']==start['environment']and len(start['environment'])==14
assert result['head']==binding['head']==sources['head']and result['tree']==binding['tree']==sources['tree']
for key in ['sourceBefore','sourceAfter']:assert result[key]['sourcePinsVerified']==198 and result[key]['clean']and result[key]['sourceToolExpectationControlPinsVerified']
assert result['exitCode']==terminal['exitCode']==0 and result['selectedCheckSatisfied']and terminal['selectedCheckSatisfied']is False
assert result['rustSummary']=={'passed':1,'failed':0,'ignored':0,'measured':0,'filtered':0}
assert result['remainingMembers']==proc['remainingMembers']==clean['remainingMembers']==[]
assert proc['ownedSession']==child['pid']==child['ownedSession']==child['ownedGroup']and clean['existsAfter']==result['ownedRootExistsAfter']==False
assert len(plan['absentAncestorConfigs'])==result['absentAncestorConfigCountAfter']==18
keys=['action','semantics','status','pre','steps','post','effects','frame_pre','frame_post','residual','attempted','committed','selectedEvent'];assert set(resume)==set(keys)
assert set(journey)=={'pre','steps','post','orderedCrossSegmentEffects'}
assert eq(resume,{k:ER[k]for k in keys});assert eq(journey,{k:EJ[k]for k in journey})
assert len(resume['steps'])==len(ue['steps'])==4 and len(journey['steps'])==10 and len(se['steps'])==6
assert eq(reopened,waiting['continuation'])and waiting['advanced']is True
assert eq(reopened['boundary'],ER['pre'])and eq(se['post'],ER['pre'])
assert reopened['logical']['cap']==W['startCap']=='23'and reopened['logical']['committed']==W['startCommitted']=='15'
assert waiting['attempted']==W['startAttempted']=='15'and complete['attempted']==W['resumeAttempted']=='8'and complete['committed']==W['completionCommitted']=='23'
assert eq(waiting['residual'],ER['residual'])and eq(complete['residual'],ER['residual'])
checks=[]
def check(id,a,b,basis):assert eq(a,b),id;checks.append({'id':id,'completeTypedObjectEqual':True,'basis':basis})
for k in keys:check('resume-'+k,resume[k],ER[k],'existing13-key whole semantic view equality')
for k in journey:check('journey-'+k,journey[k],EJ[k],'existing4-key whole semantic view equality')
check('whole-typed-reopened',reopened,waiting['continuation'],'existing typed reopened==continuation; self-consistency, not independent hash/codec oracle')
# All leaf prefix aliases retain where each financial/value/kind observation came from.
links=[]
def link(fn,prefix,target,targetprefix,reason):
 check(fn+':'+prefix,ptr(A[fn],prefix),ptr(A[target],targetprefix),reason);links.append({'file':fn,'prefix':prefix,'targetFile':target,'targetPrefix':targetprefix,'reason':reason})
for fn,base,execv,offset,segment in [(SS,'/Ok/Waiting/segment',se,0,'start'),(ZS,'/Ok/Complete/segment',ue,6,'resume')]:
 eb=base+'/execution'
 for k in ['pre','post']:
  dest='pre'if segment=='start'and k=='pre'else'post'if segment=='resume'and k=='post'else None
  if dest:link(fn,eb+'/'+k,JS,'/'+dest,'raw complete financial endpoint equals independently compared journey view')
  else:link(fn,eb+'/'+k,RS,'/pre','raw complete boundary equals independently compared suffix pre')
 for i,step in enumerate(execv['steps']):
  dest='/steps/'+str(i+offset)
  for rawkey,viewkey in [('index','sourceOrdinal'),('value','typedValue'),('snapshot','snapshot'),('effects','deltaEffects'),('statement/kind','statementKind')]:link(fn,eb+f'/steps/{i}/'+rawkey,JS,dest+'/'+viewkey,'raw record copied by prefixes adapter; exact fields independently compared in journey')
  # suffix also checked in its separate view, including full ordered effect prefixes.
  if segment=='resume':
   for viewkey in ['sourceOrdinal','typedValue','snapshot','deltaEffects','statementKind']:check(f'suffix-journey-alias-{i}-{viewkey}',resume['steps'][i][viewkey],journey['steps'][i+6][viewkey],'complete alias consistency between two independently compared views')
 # Execution.effects are separately concatenated by the existing test. Source does not separately assert each segment==flattened steps.
 effecttarget='/steps/5/derivedCrossSegmentEffectPrefix'if segment=='start'else'/effects'
 link(fn,eb+'/effects',JS if segment=='start'else RS,effecttarget,'metadata alias: whole captured segment effects equal frozen selected effect vector; existing test separately compares concatenated segment vectors')
for k in ['action','frame_pre','frame_post']:link(ZS,'/Ok/Complete/segment/'+k,RS,'/'+k,'direct extraction into independently compared resume view')
for k in ['committed','attempted','residual']:link(ZS,'/Ok/Complete/'+k,RS,'/'+k,'direct extraction into independently compared resume view')
for k in ['action','semantics']:link(ZS,'/Ok/Complete/segment/execution/'+k,RS,'/'+k,'observed raw execution identity equals selected semantic identity; action view uses segment.action, so extra raw execution.action agreement is metadata-only')
# Cross-artifact aliases to independently compared suffix frame, not additional original whole-start assertions.
link(SS,'/Ok/Waiting/segment/frame_post',RS,'/frame_pre','post-check cross-artifact pending frame equality; no independent whole-start frame oracle selected')
link(SS,'/Ok/Waiting/continuation/boundary',RS,'/pre','existing explicit boundary assertion')
link(CS,'/boundary',RS,'/pre','reopen self-consistency plus explicit original boundary assertion')
for fn,base in [(SS,'/Ok/Waiting/continuation'),(CS,'')]:
 for field in ['site','captures']:link(fn,base+'/'+field,RS,'/frame_pre/'+field,'post-check pending continuation/frame-pre alias; frozen suffix frame independently compared, full raw continuation not independently golden')
# Reconstruct views by copying raw records/concatenating observed deltas only (no source evaluator).
def observed_steps(execution,segment,key,acc):
 out=[]
 for step in execution['steps']:
  acc.extend(step['effects']);out.append({'sourceOrdinal':step['index'],'segment':segment,'statementKind':step['statement']['kind'],'typedValue':step['value'],'snapshot':step['snapshot'],'deltaEffects':step['effects'],key:list(acc)})
 return out
acc=[];rsteps=observed_steps(ue,'resume','derivedSegmentEffectPrefix',acc);acc=[];jsteps=observed_steps(se,'start','derivedCrossSegmentEffectPrefix',acc)+observed_steps(ue,'resume','derivedCrossSegmentEffectPrefix',acc)
check('complete-suffix-copy-adapter',rsteps,resume['steps'],'pure JSON copy+ordered concatenation, not evaluate/simulate')
check('complete-journey-copy-adapter',jsteps,journey['steps'],'pure JSON copy+ordered concatenation, not evaluate/simulate')
check('whole-cross-segment-effects',se['effects']+ue['effects'],journey['orderedCrossSegmentEffects'],'existing combinedEffects construction, complete raw source vectors')
# Per-leaf classification across all five complete raw objects, including empty arrays/nulls.
leaves=[]
for fn,v in A.items():
 for p,value in walk(v):
  row={'file':fn,'pointer':p,'value':value,'jsonType':type(value).__name__,'classification':'unasserted-raw-representation-or-metadata','expectedFile':None,'expectedPointer':None,'bridge':None}
  if fn in [RS,JS]:
   ep=p;ev=ER if fn==RS else EJ;assert eq(value,ptr(ev,ep));row.update(classification='independent-complete-selected-view-equality',expectedFile='EXPECTED-RESUME.json'if fn==RS else'EXPECTED-ALL-PREFIXES.json',expectedPointer=ep,bridge='whole explicit semantic view equality in unchanged passing consumer')
  else:
   options=[x for x in links if x['file']==fn and(p==x['prefix']or p.startswith(x['prefix']+'/'))]
   if options:
    x=max(options,key=lambda x:len(x['prefix']));tp=x['targetPrefix']+p[len(x['prefix']):];ev=ER if x['targetFile']==RS else EJ
    assert eq(value,ptr(A[x['targetFile']],tp))and eq(value,ptr(ev,tp));row.update(classification='captured-raw-alias-to-independent-selected-field',expectedFile='EXPECTED-RESUME.json'if x['targetFile']==RS else'EXPECTED-ALL-PREFIXES.json',expectedPointer=tp,bridge=x['reason'],aliasFile=x['targetFile'],aliasPointer=tp)
   else:
    capbase='/Ok/Waiting/continuation'if fn==SS else''if fn==CS else None
    workkey=None
    if capbase is not None and p==capbase+'/logical/cap':workkey='startCap'
    if capbase is not None and p==capbase+'/logical/committed':workkey='startCommitted'
    if fn==SS and p=='/Ok/Waiting/attempted':workkey='startAttempted'
    if workkey:
     assert eq(value,W[workkey]);row.update(classification='independent-explicit-work-aggregate-equality',expectedFile='EXPECTED-LOGICAL-WORK.json',expectedPointer='/'+workkey,bridge='explicit original cap/committed/attempted assertion; continuation copy via typed reopen equality')
    elif fn==SS and p=='/Ok/Waiting/residual':
     assert eq(value,ER['residual']);row.update(classification='independent-explicit-start-residual-equality',expectedFile='EXPECTED-RESUME.json',expectedPointer='/residual',bridge='existing explicit start_residual assertion')
    elif fn==SS and p=='/Ok/Waiting/advanced':assert value is True;row.update(classification='explicit-source-literal-assertion-not-an-independent-raw-wire-golden',bridge='existing assert!(advanced) and Waiting pattern')
    elif capbase is not None and(p.startswith(capbase+'/')or fn==CS):row.update(classification='typed-reopen-self-consistency-only-no-independent-raw-identity',bridge='whole reopened==continuation asserted and captured objects structurally equal; format/program_hash/plan_hash/logical.profile/action not independently golden here')
    elif '/statement/'in p:row.update(bridge='Only statement.kind projected/asserted; raw numeric ExprIds, binder/argument/name/layout fields retained without independent complete Statement/arena oracle')
    else:row.update(bridge='Retained raw field outside selected independent assertion; any coincidentally matching literal is not an independent raw oracle')
  leaves.append(row)
assert len(leaves)==len({(x['file'],x['pointer'])for x in leaves})
def containers(v,p=''):
 if isinstance(v,dict):
  yield {'pointer':p,'type':'object','keys':list(v)}
  for k,w in v.items():yield from containers(w,p+'/'+k.replace('~','~0').replace('/','~1'))
 elif isinstance(v,list):
  yield {'pointer':p,'type':'array','length':len(v)}
  for i,w in enumerate(v):yield from containers(w,p+'/'+str(i))
container_rows={fn:list(containers(v))for fn,v in A.items()}
write(D/'ACTUAL-LEAF-COVERAGE.json',{'definition':'Every scalar occurrence and every empty array/object occurrence in each of five complete unchanged actualfiles; array order and JSONtypes retained. Raw bytes not normalized.','counts':dict(collections.Counter(x['classification']for x in leaves)),'total':len(leaves),'leaves':leaves})
write(D/'ACTUAL-CONTAINER-COVERAGE.json',{'files':container_rows,'total':sum(len(x)for x in container_rows.values()),'meaning':'Every keyset and array cardinality/order retained; Ok/Waiting/Complete containers observed and source pattern-matched, no independent whole enum-wire golden.'})
write(D/'RAW-TO-SELECTED-VIEW-LINKS.json',{'links':links,'interpretation':'A metadata equality link is separately identified from a direct original consumer assertion. No independent whole Result/Continuation/Program/hash/codec oracle is inferred.'})
write(D/'COMPLETE-COMPARISONS.json',{'checks':checks,'allMatch':True,'sourceResultAcceptance':False,'noProductEvaluation':True,'wholeResultTags':{'start':'Ok/Waiting','resume':'Ok/Complete'},'statusBridge':'resume view literalcomplete constructed only after actual Ok Complete pattern; start Waiting/advanced source pattern. No serialized status member in raw outcomes.'})
# Complete source-independent financial summary projections from captured fields, no new expected values.
def amt(s,account):return int(next(x['amount']for x in s['balances']if x['account']==account))
def flow(ex):
 transfer=sum(int(x['amount'])for x in ex['effects']if x['kind']=='Transfer'and x['from']=='alice');fee=sum(int(x['amount'])for x in ex['effects']if x['kind']=='Fee'and x['from']=='alice')
 return {'grossDebit':str(transfer+fee),'transfer':str(transfer),'fee':str(fee),'recipientNetIncrease':str(amt(ex['post'],'merchant')-amt(ex['pre'],'merchant'))}
sf=flow(se);uf=flow(ue);startduty=next(x for x in se['post']['liabilities']if x['name']=='paid-invoice')
financial={'sourceAction':su['action'],'quantity':str(next(x for x in reopened['captures']if x['name']=='quantity')['value']['value']),'owed':str(next(x for x in reopened['captures']if x['name']=='owed')['value']['value']),'start':dict(sf,treasuryIncrease=str(amt(se['post'],'treasury')-amt(se['pre'],'treasury')),dutyCreated=str(sum(int(x['after'])-int(x['before'])for x in se['effects']if x['kind']=='Liability'and int(x['after'])>int(x['before']))),dutyDischarged=str(sum(int(x['before'])-int(x['after'])for x in se['effects']if x['kind']=='Liability'and int(x['after'])<int(x['before']))),positiveResidual=waiting['residual'],presentZeroRetained=startduty,opening=se['pre'],waitingBoundary=se['post'],effects=se['effects']),'resume':dict(uf,positiveResidual=complete['residual'],completeBoundary=ue['post'],effects=ue['effects']),'combined':{'grossDebit':str(int(sf['grossDebit'])+int(uf['grossDebit'])),'fee':str(int(sf['fee'])+int(uf['fee'])),'recipientNetIncrease':str(amt(ue['post'],'merchant')-amt(se['pre'],'merchant')),'completeBoundary':ue['post'],'orderedEffects':se['effects']+ue['effects']}}
financial['resume'].pop('transfer') if 'transfer'not in F['resume'] else None
for p,v in walk(financial):assert eq(v,ptr(F,p)),p
write(D/'FINANCIAL-PROJECTION-COMPARISON.json',{'projection':financial,'allCorrespondingFrozenFieldsEqual':True,'meaning':'Metadata arithmetic/copies of actual observed records compared to existing frozen summary; no new independent expected amounts and no production evaluator call. Descriptiveunits/preservedRows/unused-action strings not invented as actualfields.'})
# Every frozen field accounted, including aggregate-vs-per-statement work/representation gaps.
expected_coverage=[]
for fn,ev in [('EXPECTED-RESUME.json',ER),('EXPECTED-ALL-PREFIXES.json',EJ),('EXPECTED-LOGICAL-WORK.json',W),('EXPECTED-FINANCIAL-SUMMARY.json',F)]:
 for p,value in walk(ev):
  method='unselected-ancillary-schema-explanation-or-unobserved-detail';evidence=None
  if fn=='EXPECTED-RESUME.json'and p.split('/')[1]in keys:method='whole13-key independent equality';evidence={'file':RS,'pointer':p};assert eq(value,ptr(resume,p))
  elif fn=='EXPECTED-ALL-PREFIXES.json'and p.split('/')[1]in journey:method='whole4-key independent equality';evidence={'file':JS,'pointer':p};assert eq(value,ptr(journey,p))
  elif fn=='EXPECTED-LOGICAL-WORK.json'and p in ['/startCap','/startCommitted','/startAttempted','/resumeAttempted','/completionCommitted']:method='independent aggregate counter/cap compared; per-statement node counts not persisted';evidence={'aggregateValue':value}
  elif fn=='EXPECTED-FINANCIAL-SUMMARY.json':
   try:observed=ptr(financial,p)
   except (KeyError,IndexError,TypeError,ValueError):pass
   else:assert eq(value,observed);method='metadata projection equality, not a separate original test report';evidence={'financialProjectionPointer':p}
  elif fn=='EXPECTED-ALL-PREFIXES.json'and p.startswith('/startAwaitEntry/boundary'):
   ap='/Ok/Waiting/continuation/boundary'+p[len('/startAwaitEntry/boundary'):];assert eq(value,ptr(A[SS],ap));method='metadata boundary equality corroborating existing boundary assertion';evidence={'file':SS,'pointer':ap}
  expected_coverage.append({'file':fn,'pointer':p,'value':value,'method':method,'evidence':evidence})
write(D/'EXPECTED-LEAF-COVERAGE.json',{'total':len(expected_coverage),'leaves':expected_coverage,'scope':'Unselected metadata may describe financial properties already observable in full frames/effects; no raw Program/Plan/hash/ExprId or per-statement meter oracle is synthesized.'})
write(D/'OBSERVER-CORRECTION.json',{'kind':'Recorded root metadata observer mistake, not product failure/retry','evidence':'raw-actual/ROOT-ACTUAL-DISPOSITION.json#/observerCorrection','sourceSha256':'b2ce4ce50f6662ff08d79cc285190a982addc2993fc601c983a99d6ee112a4df','initialAssumption':'continuation directly under Ok','observedWholeTags':{'start':['Ok','Waiting'],'resume':['Ok','Complete']},'reportedFailure':'KeyError continuation','separateOriginalTracebackAvailableInSuppliedCycle':False,'correctedObservation':'Start continuation at/Ok/Waiting/continuation; suffix segment at/Ok/Complete/segment. Both complete raw captures unchanged.','productRerun':False,'runtimeFailureInferred':False,'noPrivateCaptureRead':True})
write(D/'RAW-COPY-MAPPINGS.json',{'files':copies,'allByteIdentical':True,'normalization':False})
counts=collections.Counter(x['classification']for x in leaves);gaprows=[x for x in leaves if x['expectedPointer']is None]
summary={'head':result['head'],'tree':result['tree'],'parent':binding['parent'],'wholeActualFiles':16,'completeCaptureFiles':5,'sourcePins':198,'originalInvoiceFixtureFiles':18,'rawLeafAndEmptyOccurrences':len(leaves),'rawContainerCount':sum(len(x)for x in container_rows.values()),'leafClasses':dict(counts),'expectedFourFileLeafOccurrences':len(expected_coverage),'completeObjectChecks':len(checks),'allChecksEqual':True,'suffixSteps':4,'journeySteps':10,'startAttempted':'15','storedCommitted':'15','cap':'23','suffixAttempted':'8','finalCommitted':'23','actualExitCode':0,'actualWallSeconds':result['wallSeconds'],'rustSummary':result['rustSummary'],'sourceReviewAdoption':False,'newProductProviderExecution':False,'parentEightScopeSatisfied':False}
write(D/'SUMMARY.json',summary)
# Human-readable full materiality report plus every unestablished raw field.
lines=['This is a complete ordinary public metadata/source-fidelity comparison, not a source/result review vote, source approval, new oracle, provider call or product rerun. The five full genuine Results/Continuation/semantic views are retained unchanged, not replaced by selected-field summaries.','',f'Exact source 0345e172faf9e601404d7cb96211bc4e0255de4c/tree ea4b5c97779788ff25ca54f51725d6f759b688d9,parent 68de. Root-provided outer session91459 was terminal0/collected. Actual whole target1pass/0failed/0ignored/0measured/0filtered,exit0,{result["wallSeconds"]:.11f}s.198source pins/7tools/expectations/control custody before/after;14controlled keys (13 original plus the capture key),offline cold target,18absent config paths,owned SID empty and owned root removed recorded. These are qualified actual receipts, not a general host/process/target theorem.','',f'All {len(checks)} complete typed object comparisons match. The 13 resume keys and four journey keys match the full frozen selected projections with JSON type-sensitive equality,array order and full key sets retained. Raw-to-view complete aliases and full reconstruction by copying raw records and concatenating captured deltas also match; no source evaluation occurred. Every one of {len(leaves)} actual scalar-or-empty-container occurrences and all {summary["rawContainerCount"]} containers is mapped. Four frozen expectation documents have {len(expected_coverage)} leaf occurrences in total, each classified; unselected fields remain visible.','', 'Actual artifact identities:','']
for row in captures['files']:lines.append('- '+row['name']+': '+str(row['bytes'])+'B SHA256 '+row['sha256'])
lines+=['','Actual wrapper structure and observer correction:','','- Start is {Ok:{Waiting:{advanced,segment,continuation,attempted,residual}}}; resume is {Ok:{Complete:{segment,committed,attempted,residual}}}. Reopened capture is the bare typed Continuation. Source destructuring supplies the success/status bridge; raw Results have no independent status member or whole serializer golden. Semantic status complete is a constructed view literal after genuine Complete matching, not an independently serialized ledger status.','- Root first assumed continuation directly under Ok, stopped with KeyError for continuation, inspected full actual tags, then corrected its metadata observer. ROOT-ACTUAL-DISPOSITION retains this mistake verbatim; OBSERVER-CORRECTION identifies actual paths. No standalone traceback was supplied in the 11 control files; none is fabricated. The product command ran once, and raw files/expectations were not changed or regenerated.','', 'Complete financial journey: initial principal 0, alice 50, merchant 0, observer 4 and treasury 0. Quantity 14 yields owed 17. Start creates duty 17, transfers 17, charges fee 2, discharges 17 to a present zero row, and writes principal 14. The boundary contains alice 31, merchant 17, observer 4, treasury 2, principal 14 and a present paid-invoice duty of 0. All asset identity fields (network local, issuer receipt-issuer, reference credit and scale 3) remain exact; scale 3 does not rescale atoms. Start gross debit 19 = 17 + 2, merchant net 17 and treasury net 2; total 54 conserved. Suffix financial debit 0 and fee 0; prior financial operations are not repeated; all four suffix snapshots preserve that boundary and positive residuals remain[]. The zero duty stays present; later-credit remains absent. The guard accepts true then emissions present-zero Bool true, absent Bool false and closed-amount UInt 0 are independently compared typed values/effects. All ten whole prefixes retain state, balance and duty presence, identities, order, deltas and cumulative history.','', '| ordinal | segment | statement kind | typed value | delta kinds | cumulative effects |','|---:|---|---|---|---|---:|']
for stp in journey['steps']:lines.append('| '+' | '.join([str(stp['sourceOrdinal']),stp['segment'],stp['statementKind'],json.dumps(stp['typedValue'],separators=(',',':')),','.join(x['kind']for x in stp['deltaEffects'])or'[]',str(len(stp['derivedCrossSegmentEffectPrefix']))])+' |')
lines+=['','Reference work and frame custody:','','- Actual stored cap 23, committed 15, start attempted 15, suffix attempted 8 and final committed 23 compare with frozen aggregates. Existing resume call supplies None override, so source uses stored cap; no reset or new metering contract is invented. Independent work derivation is 15 start + 8 suffix = 23 under stated fresh-node/dispatch/await rules. No per-statement meter visit counters, full Program/Plan/cost allocation or source ExprId mapping is independently captured/established; observed aggregate conformance does not independently prove every predicted node cost. Native/circuit/chain fees/allocation/wall time/authenticated lifetime/proof remain separate.','- Whole reopened equals start continuation (captured relation and original typed assertion). Boundary matches independent suffix prestate. Raw pending frame/site/captures cross-links to independently compared frame_pre; completed frame_post site 0/captures [] independently compared. Reopen self-consistency, observed hash strings and metadata equality are not independently specified Program/Plan/hash/canonicalcodec/authentication/ledger acceptance.','- The scenario is owned and moved into stage::start; this consumer has no borrowed Scenario immutability assertion. Do not inherit escrow borrow claims. Full initial/financial scenario fields are compared through genuine execution pre/steps/post; no separately persisted Program arena or original Scenario after object is supplied.','', 'Complete independently unestablished or source-literal-only raw leaves (every pointer/value; typed Step.value and statement.kind ARE independently projected here, unlike escrow):','']
for row in gaprows:lines.append('- '+row['file']+row['pointer']+' = '+json.dumps(row['value'],ensure_ascii=False)+'; '+row['classification']+'; '+str(row['bridge']))
lines+=['','Remaining exact qualifications:','','- Full raw Result/Continuation wire is not independently golden. Unselected statement arguments/numeric ExprIds/binder/asset strings/arena are not a complete independent Statement expectation, even when source strings appear obvious. All raw fields remain in coverage/raw captures; keysets and Ok/Waiting/Complete shapes are observed/source-pattern matched, not a canonical wire theorem. Source clone/reopen equality is meaningful internal relation and never promotes observed Program/Plan hashes into an independent identity oracle.','- Original 18 fixture files, freeze, derivation, provenance and UNKNOWN and normative excerpts retained. Provenance discloses earlier original InvoiceReceipt start oracle and conditional resume consequences and related Core2 report design and Source6 exposure; not globally blind. This derivation wave claimed only frozen inputs and normative documents, no current author assertions, evaluator outputs or implementation body reads. Current integrator/helper/metadata author have source, expected-data and actual exposure; metadata comparison does not become a new independent oracle.','- EXPECTED-RESUME ancillary format/ordinaryCondition/financialSuffix/presence/statementExprIds, EXPECTED-ALL-PREFIXES startAwaitEntry/scope, financial summary descriptions and detailed work node/cost vectors are not all separately selected original test assertions. Whole selected frames/emissions preserve zero-vs-absent semantics. Every ancillary leaf is explicitly classified rather than removed or blindly counted as independent whole-wire pass.','- Existing capture helper fs::write uses pretty serde JSON without an added LF, create_new or sync_all. Fresh empty capture directory is provided by the root helper; writer guarantees are not silently upgraded. Three genuine raw objects are captured immediately after their operations; both derived views before final assertions. Raw files existing alone do not prove assertions; complete test stdout/exit0 supplies that bridge.','- TERMINAL selectedCheckSatisfied false precedes capture hashes, census and source postchecks; final RESULT true is later temporal state, not inconsistent product outcomes. Whole INVOICE-CAPTURES, files, controls, streams and root disposition retained. No retained binary digest/identity attestation was recorded; owned cold target removed and no reconstruction attempted. All 198 public source copy hashes, control hashes, original 18 aliases and 133 external original pins checked as metadata; no private/owned cache, live process or native reads.','- No actual public CLI subprocess in this consumer; no original public stdout/report format golden is established. Existing pure host canonical serialization/Sha256 stage calls are part of the inspected production path, not a security/crypto/native qualification. No SDK/VM/proof/signing/key/wallet/ledger/native action or source acceptance occurred here.','- Original five new and three overlapping tests in the full eight-test scope remains. Escrow and this invoice consumer have separate scoped observations; three payout tests plus one Waiting and two reserve tests retain required separate custody and unrun status. No eight-of-eight family acceptance or parent goal closed. All 148 requirements, 292 scenarios and 20 families,U/M/MC/SP/G,ACTUS277/18/32/DS01-07,DeFi72/DA01-24,all five composition operators and financial/proof/native/target exits preserved. A full nonauthor source/result audit remains separate.','', 'Machine maps are exhaustive metadata, not substitute votes: ACTUAL-LEAF-COVERAGE and ACTUAL-CONTAINER-COVERAGE cover every actual field/container; RAW-TO-SELECTED-VIEW-LINKS states exact direct-vs-postcheck bridges; EXPECTED-LEAF-COVERAGE retains every frozen selected/ancillary field; FINANCIAL-PROJECTION-COMPARISON compares existing financial summary numerics/frames against captured observations; RAW-COPY-MAPPINGS and ALIAS-AND-CONTROL-CHECKS preserve all bytes/provenance. Full raw objects and source are mandatory for future substantive review.']
(D/'REPORT.md').write_text('\n'.join(lines).replace('financialdeb it0','financialdebit0')+'\n')
files=[{'path':str(p.relative_to(D)),'bytes':p.stat().st_size,'sha256':sha(p.read_bytes())}for p in sorted(D.rglob('*'))if p.is_file()and p.name not in ['INVENTORY.json','FREEZE.sha256']];write(D/'INVENTORY.json',{'files':files,'metadataOnly':True})
files=sorted(p for p in D.rglob('*')if p.is_file()and p.name!='FREEZE.sha256');(D/'FREEZE.sha256').write_text(''.join(sha(p.read_bytes())+'  '+str(p.relative_to(D))+'\n'for p in files))
for line in(D/'FREEZE.sha256').read_text().splitlines():h,p=line.split('  ',1);assert sha((D/p).read_bytes())==h
print(json.dumps({'summary':summary,'reportSha256':sha((D/'REPORT.md').read_bytes()),'freezeSha256':sha((D/'FREEZE.sha256').read_bytes()),'payloads':len(files)}))
