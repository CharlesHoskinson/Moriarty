"""Finite complete public JSON/hash/text comparison; no subject import or evaluation."""
from pathlib import Path
import hashlib,json,shutil,collections
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017');D=Path(__file__).parent
Q=R/'escrow-full-prefix-normal90-preparation-20261018';C=R/'escrow-full-prefix-normal90-check-cycle-20261018'
def sha(b):return hashlib.sha256(b).hexdigest()
def load(p):return json.loads(p.read_bytes())
def write(p,v):p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(v,indent=2,ensure_ascii=False)+'\n')
def canonical(v):return json.dumps(v,sort_keys=True,separators=(',',':'),ensure_ascii=False)
def eq(a,b):return canonical(a)==canonical(b)
def ptr(v,path):
 for k in path.split('/')[1:] if path else []:v=v[int(k)] if isinstance(v,list) else v[k.replace('~1','/').replace('~0','~')]
 return v
def walk(v,p=''):
 if isinstance(v,dict) and v:
  for k,w in v.items():yield from walk(w,p+'/'+k.replace('~','~0').replace('/','~1'))
 elif isinstance(v,list) and v:
  for i,w in enumerate(v):yield from walk(w,p+'/'+str(i))
 else:yield p,v
rawmap=[]
def copy(src,dst):
 dst.parent.mkdir(parents=True,exist_ok=True);b=src.read_bytes();dst.write_bytes(b);assert dst.read_bytes()==b
 rawmap.append({'source':str(src),'copy':str(dst.relative_to(D)),'bytes':len(b),'sha256':sha(b),'byteIdentical':True})
for p in sorted(C.iterdir()):
 if p.is_file():copy(p,D/'raw-actual'/p.name)
assert len([x for x in rawmap if x['copy'].startswith('raw-actual/')])==12  # original11 + additive root disposition
assert sha((C/'ROOT-ACTUAL-DISPOSITION.json').read_bytes())=='f0f56499361684ff7f8ad45992872ec3328bfcd53cd69d2c8c5922ff4623606d'
assert sha((C/'RESULT.json').read_bytes())=='fc298a6ce62d56b3c256a749c6571ee418e304e0a4280d37682548618591c450'
sources=load(Q/'SOURCE-PINS.json');assert len(sources['files'])==199
for row in sources['files']:
 src=Q/'current-source'/row['path'];assert src.stat().st_size==row['bytes'] and sha(src.read_bytes())==row['sha256'];copy(src,D/'source'/row['path'])
for n in ['BOUND-CANDIDATE.json','SOURCE-PINS.json','EXPECTATION-PINS.json','TOOL-PINS.json','EFFECT-CLOSURE.json','ROOT-SOURCE-BINDING.json','SOURCE-EXPOSURE.json','FIVE-CONSUMER-MAPPINGS.json','RUNTIME-REQUIREMENTS.json','ENVIRONMENT-DIFF.json','STATIC-INSPECTION.json','run_check.py','FREEZE.sha256','helper-vs-inspected-normal90.diff']:
 copy(Q/n,D/'check-preparation'/n)
for n in ['FULL-EIGHT-SCOPE-CUSTODY.json','PLAN.md','NEW-FIVE-CONSUMERS.json']:
 copy(Q/'plan-custody'/n,D/'parent-scope'/n)
binding=load(Q/'BOUND-CANDIDATE.json');controls=[]
assert sha((Q/'run_check.py').read_bytes())==binding['driverSha256']
for n,h in binding['controlHashes'].items():assert sha((Q/n).read_bytes())==h;controls.append({'path':str(Q/n),'sha256':h,'verified':True})
expect=load(Q/'EXPECTATION-PINS.json');aliases=[]
for row in expect['externalOriginalAndSourcePacketFiles']:
 p=Path(row['path']);assert p.stat().st_size==row['bytes'] and sha(p.read_bytes())==row['sha256']
 # Retain exact external original public controls/expectations as identified, no traversal.
 copy(p,D/'original-external'/str(len(aliases)).zfill(3)/p.name);aliases.append({'kind':'externalOriginalAndSourcePacketFiles',**row,'verified':True})
for row in expect['copiedIndependentFiles']:
 p=Path(row['originalPath']);b=(D/'source'/row['repositoryPath']).read_bytes();assert b==p.read_bytes() and sha(b)==row['sha256'];aliases.append({'kind':'copiedIndependentFiles',**row,'byteIdentical':True})
write(D/'ALIAS-AND-CONTROL-CHECKS.json',{'controls':controls,'aliases':aliases,'all199SourceCopiesMatchFrozenPins':True,'sourcePinBasis':'Frozen helper copies hash-checked to199 SOURCE-PINS; actual before/after asserts same hashes at0345. No live Git/provider/product action and no independently attested kernel timestamps inferred.'})
A=load(C/'escrow-execution.json');B=D/'source/crates/fixtures/utxo-escrow-partial-release';E=load(B/'independent/COMPLETE-PREFIXES.expected.json');F=load(B/'independent/FINANCIAL-ACCOUNTING.expected.json');O=load(B/'independent/ORDERED-EFFECTS.expected.json');S=load(B/'normal.scenario.json')
result=load(C/'RESULT.json');terminal=load(C/'TERMINAL.json');cap=load(C/'EXECUTION-CAPTURE.json');start=load(C/'START.json');plan=load(C/'CHECK-PLAN.json');child=load(C/'CHILD.json');clean=load(C/'CLEANUP.json');proc=load(C/'PROCESS-CLEANUP.json')
assert sha((C/'escrow-execution.json').read_bytes())=='3871ddf18927ab7d466cd196ef18122c82d7f28aeac37f036ce75a24756e613e'
assert (C/'escrow-execution.json').stat().st_size==50925 and cap==result['executionCapture']
assert cap['sha256']==sha((C/'escrow-execution.json').read_bytes()) and cap['bytes']==50925
for n,k in [('STDOUT.log','stdoutSha256'),('STDERR.log','stderrSha256')]:assert result[k]==sha((C/n).read_bytes())
assert result['head']==binding['head']==sources['head'] and result['tree']==binding['tree']==sources['tree']
assert result['sourceBefore']['sourcePinsVerified']==199 and result['sourceAfter']['sourcePinsVerified']==199
assert result['sourceBefore']['clean'] and result['sourceAfter']['clean'] and result['sourceBefore']['sourceToolExpectationControlPinsVerified'] and result['sourceAfter']['sourceToolExpectationControlPinsVerified']
assert plan['bindingSha256']==sha((Q/'BOUND-CANDIDATE.json').read_bytes())
assert start['argv']==plan['argv'] and start['environment']==plan['environment'] and len(start['environment'])==14
assert result['selectedCheckSatisfied'] and result['rustSummary']=={'passed':1,'failed':0,'ignored':0,'measured':0,'filtered':0}
assert result['exitCode']==terminal['exitCode']==0 and terminal['selectedCheckSatisfied'] is False
assert proc['ownedSession']==child['ownedSession']==child['pid']==child['ownedGroup']==1618129
assert proc['remainingMembers']==clean['remainingMembers']==result['remainingMembers']==[]
assert clean['existsAfter']==result['ownedRootExistsAfter']==False
assert len(plan['absentAncestorConfigs'])==result['absentAncestorConfigCountAfter']==18
assert len(A['steps'])==len(E['prefixes'])==11 and len(A['effects'])==6
assert list(A)==['action','effects','post','pre','semantics','steps']
# All complete compared subtrees, preserving arrays/keysets/type identity.
checks=[]
def check(id,actual,expected,ap,ep,bridge):
 match=eq(actual,expected);checks.append({'id':id,'actualPointer':ap,'expectedPointer':ep,'completeTypedObjectEqual':match,'bridge':bridge});assert match,id
for k in ['pre','post','effects','semantics','action']:check(k,A[k],E[k],'/'+k,'/'+k,'direct existing consumer assert_eq')
assert eq(A['effects'],O)
cumulative=[]
for i,(a,e) in enumerate(zip(A['steps'],E['prefixes'])):
 for ak,ek in [('index','index'),('snapshot','snapshot'),('effects','stepEffects')]:check(f'prefix{i}-{ak}',a[ak],e[ek],f'/steps/{i}/{ak}',f'/prefixes/{i}/{ek}','direct existing consumer assert_eq')
 cumulative.extend(a['effects']);check(f'prefix{i}-cumulative',cumulative,e['cumulativeEffects'],f'concatenate(/steps/0..{i}/effects)',f'/prefixes/{i}/cumulativeEffects','existing consumer ordered cumulative vector assertion')
check('flattened-full-effects',cumulative,E['effects'],'concatenate(/steps/0..10/effects)','/effects','existing consumer final cumulative equality')
# Full actual leaf and empty-container coverage: never drop Statement/value or representation leaves.
coverage=[]
for p,v in walk(A):
 ep=None;classification='unasserted-captured-raw';reason='No independent financial expectation/assertion for this raw field.'
 seg=p.split('/')
 if seg[1] in ['pre','post','effects','semantics','action']:
  ep=p;classification='direct-complete-independent-financial-or-identity-equality';reason='Covered by whole subtree/value assert_eq and metadata exact comparison.'
 elif seg[1]=='steps':
  i=int(seg[2]);k=seg[3]
  if k in ['index','snapshot','effects']:
   ep='/prefixes/'+str(i)+'/'+{'index':'index','snapshot':'snapshot','effects':'stepEffects'}[k]+('/'+'/'.join(seg[4:]) if len(seg)>4 else '')
   classification='direct-complete-independent-financial-or-identity-equality';reason='Whole snapshot/index/delta assertion; effect is additionally included in ordered cumulative history.'
  elif k=='statement':reason='Captured actual Statement/tag/argument/ExprId, deliberately outside frozen financial predicate; no independent IR/Statement/arena expectation. Source-only observation is not an asserted oracle bridge.'
  elif k=='value':reason='Captured actual typed Step.value; deliberately outside frozen financial predicate. No independent typed expression-result assertion.'
  else:raise AssertionError('Unexpected actual Step key '+k)
 else:raise AssertionError('Unknown actual root field '+p)
 row={'actualPointer':p,'actualValue':v,'actualJsonType':type(v).__name__,'classification':classification,'expectedPointer':ep,'assertionBridge':reason}
 if ep is not None:row['frozenExpectedValue']=ptr(E,ep);row['typedValueEqual']=eq(v,ptr(E,ep));assert row['typedValueEqual']
 coverage.append(row)
assert len(coverage)==len({x['actualPointer'] for x in coverage})
# Cover every expected leaf including intentionally unasserted source text/status/schema metadata.
ec=[]
for p,v in walk(E):
 seg=p.split('/');ap=None;method=None
 if seg[1] in ['pre','post','effects','semantics','action']:ap=p;method='direct captured complete equality'
 elif seg[1]=='prefixes':
  i=int(seg[2]);k=seg[3]
  if k in ['index','snapshot','stepEffects']:ap=f'/steps/{i}/'+{'index':'index','snapshot':'snapshot','stepEffects':'effects'}[k]+('/'+'/'.join(seg[4:]) if len(seg)>4 else '');method='direct captured complete equality'
  elif k=='cumulativeEffects':
   cum=[ef for st in A['steps'][:i+1] for ef in st['effects']];assert eq(ptr(cum,'/'+'/'.join(seg[4:]) if len(seg)>4 else ''),v);method='captured deltas concatenated in order; existing cumulative assertion'
  elif k=='sourceStatement':method='UNASSERTED independent descriptive source text; existing test does not inspect Statement/ExprId'
  else:raise AssertionError(k)
 elif seg[1] in ['language','agreement']:method='Program-language/name assertions and passing-test bridge only; Program not independently captured by this file'
 elif seg[1]=='expectedLocalStatus':method='simulate.expect plus passing compiled test/exit0 bridge; no serialized accepted status field'
 elif seg[1]=='format':method='oracle metadata marker, not a subject assertion'
 else:raise AssertionError(p)
 row={'expectedPointer':p,'frozenValue':v,'method':method,'actualPointer':ap}
 if ap is not None:row['capturedValue']=ptr(A,ap);row['typedValueEqual']=eq(v,ptr(A,ap));assert row['typedValueEqual']
 ec.append(row)
# Metadata-only projections from the captured financial objects, compared to unchanged accounting summary.
def rows(s,ref):return [x for x in s['balances'] if x['asset']['reference']==ref]
def bal(s,ref,acct):return int(next(x['amount'] for x in rows(s,ref) if x['account']==acct))
def duty(s,name):return next(x for x in s['liabilities'] if x['name']==name)
pre,post=A['pre'],A['post'];flows={}
for ref in ['CASH','BONUS']:
 rs=rows(pre,ref); identity=rs[0]['asset'];account=F[ref];assert eq(identity,account['identity'])
 transfer=[x for x in A['effects'] if x['kind']=='Transfer' and x['asset']==identity];fees=[x for x in A['effects'] if x['kind']=='Fee' and x['asset']==identity]
 obs={'identity':identity,'openingTotal':str(sum(int(x['amount']) for x in rs)),'closingTotal':str(sum(int(x['amount']) for x in rows(post,ref))),'grossDebit':str(sum(int(x['amount']) for x in transfer+fees if x['from']=='escrow')),'fee':str(sum(int(x['amount']) for x in fees)),'balances':[{'account':x['account'],'opening':x['amount'],'closing':str(bal(post,ref,x['account'])),'presentAfter':True} for x in rs]}
 if ref=='CASH':obs.update(payer=transfer[0]['from'],payerOpening=str(bal(pre,ref,'escrow')),payerAfterPayout=str(bal(A['steps'][4]['snapshot'],ref,'escrow')),payerClosing=str(bal(post,ref,'escrow')),payoutDebit=str(sum(int(x['amount']) for x in transfer if x['from']=='escrow')),feeDebit=str(sum(int(x['amount']) for x in fees if x['from']=='escrow')),merchantNetReceipt=str(bal(post,ref,'merchant')-bal(pre,ref,'merchant')),treasuryNetReceipt=str(bal(post,ref,'treasury')-bal(pre,ref,'treasury')))
 else:obs['allFourRowsPreserved']=eq(rs,rows(post,ref)) and len(rs)==4
 for k,v in obs.items():assert eq(v,account[k]),ref+'/'+k
 flows[ref]=obs
liab={'opening':pre['liabilities'],'created':[],'accrued':[],'explicitlyDischarged':[{'name':x['name'],'amount':str(int(x['before'])-int(x['after']))} for x in A['effects'] if x['kind']=='Liability' and int(x['after'])<int(x['before'])],'closing':post['liabilities'],'unpaidResidual':duty(post,'merchant-due')['amount'],'unrelatedWarrantyPreserved':duty(post,'unrelated-warranty')['amount']}
for k,v in liab.items():assert eq(v,F['liabilities'][k])
assert eq(pre['state'],F['state']['opening']) and eq(post['state'],F['state']['closing'])
projections={'CASH':flows['CASH'],'BONUS':flows['BONUS'],'liabilities':liab,'state':{'opening':pre['state'],'closing':post['state']},'scope':'Post-check JSON arithmetic only, not production execution or independently authored new expected values. Original test asserts complete financial raw subtrees, not this accounting file as a distinct report. Created/accrued empty classification finite captured Liability decrease only; not general source authority/provenance. Captured StatementDischarge unasserted independently.'}
write(D/'FINANCIAL-PROJECTION-COMPARISON.json',projections)

accounting_coverage=[]
for pointer,value in walk(F):
 try:
  observed=ptr(projections,pointer)
 except (KeyError,IndexError,TypeError,ValueError):
  accounting_coverage.append({'expectedPointer':pointer,'frozenValue':value,'classification':'explanatory-schema-or-scope-text-not-a-captured-field','capturedOrProjectedValue':None})
 else:
  assert eq(value,observed),pointer
  accounting_coverage.append({'expectedPointer':pointer,'frozenValue':value,'classification':'post-check-captured-financial-projection-equality-not-an-additional-original-test-assertion','capturedOrProjectedValue':observed,'typedEqual':True})
write(D/'ACCOUNTING-EXPECTED-LEAF-COVERAGE.json',{'total':len(accounting_coverage),'leaves':accounting_coverage,'scope':'Every frozen accounting-file scalar/empty-container accounted, including explanation/schema text not projected. This is source-independent existing expected-file comparison, not a new oracle or new subject run.'})

prefix_table=[]
for i,step in enumerate(A['steps']):
 sn=step['snapshot'];prefix_table.append({'index':i,'expectedSourceDescriptionUnasserted':E['prefixes'][i]['sourceStatement'],'actualStatementUnasserted':step['statement'],'actualStepValueUnasserted':step['value'],'cashEscrow':str(bal(sn,'CASH','escrow')),'cashMerchant':str(bal(sn,'CASH','merchant')),'cashTreasury':str(bal(sn,'CASH','treasury')),'paid':sn['state']['paid']['value'],'fees':sn['state']['fees']['value'],'merchantDue':duty(sn,'merchant-due')['amount'],'stateFields':len(sn['state']),'balanceRows':len(sn['balances']),'dutyRows':len(sn['liabilities']),'deltaEffects':step['effects'],'cumulativeEffectCount':len(E['prefixes'][i]['cumulativeEffects']),'wholeSnapshotEqualsFrozen':True,'wholeDeltaAndCumulativeEqualsFrozen':True})
write(D/'PREFIX-BY-PREFIX.json',prefix_table)
counts=collections.Counter(x['classification'] for x in coverage);unasserted=[x for x in coverage if x['expectedPointer'] is None]
write(D/'ACTUAL-LEAF-COVERAGE.json',{'actualSha256':cap['sha256'],'definition':'Every scalar occurrence plus every empty object/array occurrence. No object member omitted; nonempty container keysets/arraylengths retained separately. No normalization or product evaluation. Equality is typed canonical JSONobject equality with array order intact; rawbytes untouched.','counts':dict(counts),'totalLeafAndEmptyContainerOccurrences':len(coverage),'leaves':coverage})
def containers(v,p=''):
 if isinstance(v,dict):
  yield {'pointer':p,'kind':'object','keys':list(v)}
  for k,w in v.items():yield from containers(w,p+'/'+k.replace('~','~0').replace('/','~1'))
 elif isinstance(v,list):
  yield {'pointer':p,'kind':'array','length':len(v)}
  for i,w in enumerate(v):yield from containers(w,p+'/'+str(i))
write(D/'ACTUAL-CONTAINER-COVERAGE.json',{'containers':list(containers(A)),'meaning':'Complete member/keyset and array cardinality/ordering custody; presence is not a claim of independent expectation for unasserted Statement/value.'})
write(D/'EXPECTED-LEAF-COVERAGE.json',{'total':len(ec),'leaves':ec})
write(D/'COMPLETE-COMPARISONS.json',{'checks':checks,'orderedEffectsAuxiliaryAliasEquals':True,'allChecksEqual':True,'sourceAcceptance':False,'newOracleGenerated':False,'borrowImmutability':'Only the compiled scenario==original assertion plus passing actual summary; no separately persisted after-Scenario. Actual Execution/pre is not a standalone borrowed-input immutability capture.','status':'No serialized actual status; simulate.expect and test ok/exit0 establish this normal local call succeeded under the inspected test, not target or general acceptance.'})
write(D/'RAW-COPY-MAPPINGS.json',{'files':rawmap,'noNormalizedCopies':True})
summary={'head':result['head'],'tree':result['tree'],'parent':binding['parent'],'actualFiles':12,'originalActualFiles':11,'additiveRootDisposition':True,'actualExecutionBytes':50925,'actualExecutionSha256':cap['sha256'],'sourcePins':199,'independentCopiedFixtureFiles':len(expect['copiedIndependentFiles']),'actualLeafAndEmptyContainerOccurrences':len(coverage),'directIndependentFinancialOrIdentityLeaves':len(coverage)-len(unasserted),'unassertedCapturedLeaves':len(unasserted),'actualContainerCount':len(list(containers(A))),'frozenCompleteOracleLeafOccurrences':len(ec),'completeObjectChecks':len(checks),'allCompleteObjectChecksEqual':True,'summary':result['rustSummary'],'wallSeconds':result['wallSeconds'],'exitCode':0,'borrowedScenarioOnlyAssertionBridge':True,'nativeProofLedgerAcceptance':False,'sourceReviewOrAdoption':False,'productOrProviderExecutionByThisTask':False,'relatedExposurePreserved':True}
write(D/'SUMMARY.json',summary)
# Fully readable report includes every uncovered raw leaf; complete financial vector details preserved in mandatory original raw files.
lines=['This is an ordinary metadata/source-fidelity report, not a source/result review vote, new oracle, runtime rerun or adoption. It examines the complete50925-byte captured Execution and complete57905-byte frozen expectation, not success flags alone. All raw objects/controls/source retained byteexact.','',f'Exact source HEAD{result["head"]}, tree{result["tree"]}, parent{binding["parent"]}. Root-provided outer session98188 was terminal0/collected; actual child/session/group1618129 is independently recorded in CHILD/PROCESS-CLEANUP. The single ordinary90TERM+5KILL,parent95 check completed exit0 in{result["wallSeconds"]:.11f}s;1passed,0failed/ignored/measured/filtered. No new check ran here.','',f'All{len(checks)} complete typed object comparisons match: actual pre/post/semantics/action/effects;11 indices, whole snapshots and per-step effects;11 ordered cumulative vectors; final concatenation. The auxiliary ordered-effect oracle also equals all6 actual effects. Every array order and full identity/row/state/duty presence is retained. Every one of{len(coverage)} actual scalar-or-empty-container occurrences is classified;{len(coverage)-len(unasserted)} map to exact frozen financial/identity equality and{len(unasserted)} captured Statement/Step.value leaves have no independent asserted expectation. Full keysets/cardinalities for every container are separately preserved, not discarded.','', 'The whole13 financial frames (pre+11after-statements+post) each retain9balance rows,4state fields and2positive duties. No absent/zero-duty transition is exercised. Opening supplied paid9/fees4/marker88/epoch6 are used; closing paid50/fees7 with marker88/epoch6 unchanged. CASH escrow137→96→93, merchant11→52, treasury5→8, observer23/reserve19 fixed. BONUS escrow7/merchant13/observer29/reserve37 unchanged throughout. CASH195 and BONUS86 conserved separately. Frozen-accounting comparison of captured objects confirms transfer41+fee3=gross44, merchantnet41/treasurynet3; duty97 remains through payment+fee, explicit discharge prefix6 reduces it to56, unrelated warranty8 remains. Financial-accounting summary is a post-check projection comparison, not an extra original test report or new independent expectation.','', 'Prefix-by-prefix complete comparison (table shows only a readable index; all financial fields/deltas/history are in unchanged raw objects and the exhaustive machine maps):','','| index | escrow CASH | merchant | treasury | paid | fees | due | delta kinds | cumulative effects |','|---:|---:|---:|---:|---:|---:|---:|---|---:|']
for x in prefix_table:lines.append('| '+ ' | '.join(str(v) for v in [x['index'],x['cashEscrow'],x['cashMerchant'],x['cashTreasury'],x['paid'],x['fees'],x['merchantDue'],','.join(y['kind'] for y in x['deltaEffects']) or '[]',x['cumulativeEffectCount']])+' |')
lines+=['','Complete unasserted captured leaves (every pointer/value, no Step/Statement/ExprId dropped):','']
for x in unasserted:lines.append('- '+x['actualPointer']+' = '+json.dumps(x['actualValue'],ensure_ascii=False)+'; '+x['assertionBridge'])
lines+=['','Other exact gaps and bridges:','','- Step sourceStatement strings in the frozen oracle are descriptive and are not compared with actual Statement meaning. Program language/name are asserted against frozen literals by the passing test, but fullProgram/ExprIds/arena are not captured. Captured Statement kinds/arguments/ExprIds and typed results remain unasserted independently even when they look plausible. No expression graph/cache/mapping or wholeProgram/compiler correctness inference.','- expectedLocalStatus=accepted is an oracle label; actualExecution has no status field. Existing simulate(...).expect plus compiled test ok/exit0 supports this one local success. The source has no public CLI invocation, report envelope, stdout JSON codec, admission/authentication or staged composition route.','- Borrowed Scenario equality is asserted at the end of the passing consumer, but no separate after-Scenario capture exists. pre/snapshot equality alone does not independently prove borrow immutability; the report relies on the explicit compiled assertion bridge and scoped pass.','- Raw actual50925B and expected57905B are different documents/shapes. Actual has steps with unasserted statement/value; oracle has prefixes/sourceStatement/cumulativeEffects plus format/language/agreement/expectedLocalStatus. Object member formatting/order and whole raw bytes are not compared as an independent expected wire contract. Actual arrays in the selected full financial subtrees compare exactly under disclosed AssetId/account and duty-name ordering premises. Raw bytes/hash are retained; no canonical identity/digest/serialization theorem.','- No retained testbinary digest or independently attested binary identity was recorded; owned coldtarget was removed. Source/tool/prepost/control bindings and actual compiler/test stdout/stderr support the scoped check; no deleted binary reconstruction or stronger identity claim is made.',
'- No logical cap, logical work counters, meter, costReport, consumed/attempted totals or native cost is present or measured. All scalar amounts are source tokens; fee3 is not an external chain/proof/SDK fee. No proof/SDK/VM/signing/key/wallet/native/ledger operation was selected. Ordinary k256 dependency compilation is visible in stderr and is not hidden/relabelled as a signing call.','- Original oracle FREEZE recorded2026-10-18T13:56:49.266372Z at64c6, before root checks/author outputs; ac6c is the separate guarded specimen, never this escrow freeze. Escrow independent complete expectation SHA00633e… and FREEZE.jsonc3827f… preserved. Original author disclosed related exchange/Core/Reserve/Source6 exposure and supplied ordering/DTO premises, no global blindness; no this-specimen evaluator/authored-output read or product run claimed. Current author/integrator/helper and this metadata author have implementation+frozen-data+actual exposure; this post-check analysis does not become a new independent oracle.','- Complete source/test body is unchanged and capture create_new/write_all/sync_all precedes expectation assertions. Actual helper persists TERMINAL first, captureexistence/size/hash next, then census, summary/sourcepostchecks/finalRESULT. TERMINAL selectedCheckSatisfiedfalse is temporal pre-postcheck state; finalRESULTtrue is not a contradiction. Captured Execution existing before assertions is not alone evidence assertions passed; whole actualstdout/exit0 supplies that bridge.','- SourceBefore and SourceAfter bind exactclean0345/treeea4b,199pins and tools/expectations/controls;14-key environment includes only original13 plus fixedcapturepath;18absentconfig paths, offlinecoldtarget. File/control/alias hashes verified here from frozen public copies; no private/ownedcache read, no target reconstruction/liveprocess check, no newkernel timestamps. RemainingSIDmembers[] and ownedrootremoved are actual recorded cleanup facts, not a global detached-process exclusion theorem.','- All original eight-scope obligations remain: this escrow1is observed, invoice1+payout3 remain required separate consumers and three overlaps (WaitingProgram/Plan1+reserve library/CLI2) retain separate custody; no aggregate8/8 acceptance. All148requirements/292scenarios/20families, U/M/MC/SP/G,ACTUS277/18/32/DS01-07,DeFi72/DA01-24 and allfivecompositionoperators, financial/proof/native/target exits remain open. Historical UNRUN preparation records are preserved rather than rewritten. Fresh independent nonauthor source/result audit remains separate.','', 'Read the complete raw files and source for substantive future review. ACTUAL-LEAF-COVERAGE, ACTUAL-CONTAINER-COVERAGE, EXPECTED-LEAF-COVERAGE and ACCOUNTING-EXPECTED-LEAF-COVERAGE are exhaustive metadata maps, not surrogate source approval. COMPLETE-COMPARISONS and FINANCIAL-PROJECTION-COMPARISON preserve each checked object/projection. RAW-COPY-MAPPINGS and ALIAS-AND-CONTROL-CHECKS identify all exact aliases/hashes. SUMMARY gives counts only; this report does not rely on summary flags as a substitute for complete comparison.']
(D/'REPORT.md').write_text('\n'.join(lines)+'\n')
rows=[]
for p in sorted(x for x in D.rglob('*') if x.is_file() and x.name not in ['INVENTORY.json','FREEZE.sha256']):rows.append({'path':str(p.relative_to(D)),'bytes':p.stat().st_size,'sha256':sha(p.read_bytes())})
write(D/'INVENTORY.json',{'files':rows,'noProductProviderExecution':True,'rawObjectsUnchanged':True})
allfiles=sorted(x for x in D.rglob('*') if x.is_file() and x.name!='FREEZE.sha256');(D/'FREEZE.sha256').write_text(''.join(sha(p.read_bytes())+'  '+str(p.relative_to(D))+'\n' for p in allfiles))
for line in (D/'FREEZE.sha256').read_text().splitlines():h,p=line.split('  ',1);assert sha((D/p).read_bytes())==h
print(json.dumps({'summary':summary,'reportSha256':sha((D/'REPORT.md').read_bytes()),'freezeSha256':sha((D/'FREEZE.sha256').read_bytes()),'payloadFiles':len(allfiles)}))
