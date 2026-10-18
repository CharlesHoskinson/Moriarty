import json, pathlib, hashlib, shutil, subprocess
R=pathlib.Path('/home/charl/moriarty-c2-opus55-restored-20261017')
P=R/'cli-profile-guidance-source-result-review-20261018'
O=R/'cli-profile-guidance-main8269-source-result-review-20261018'
J='/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018'
H='f2975f1fbfbaf060207a9d26331ad52745ce5247'; T='3db626c5806d44d7f0a2bf49936d146d66549196'; B='8269b5b7ceeec5169c706aa31ea33f390d49c969'
OLD='d7a6da06f24096a450cd7c55b98235d8bf920d97'
def sha(b):return hashlib.sha256(b).hexdigest()
def dump(p,x):p.write_text(json.dumps(x,indent=2)+'\n')
def readj(p):return json.loads(p.read_text())
def git(*a):return subprocess.check_output(['git',*a],cwd=J)
def obj(path):return git('show',H+':'+path)
def ptr(x,p):
 for k in p.strip('/').split('/'):x=x[k.replace('~1','/').replace('~0','~')]
 return x
assert git('rev-parse',H+'^{tree}').decode().strip()==T
changed=git('diff','--name-only',B,H).decode().splitlines()
oldpins=readj(P/'CURRENT-SOURCE-PINS.json'); assert changed==sorted(oldpins['changedPaths'])
for d in ['source-preparation']:
 shutil.copytree(P/d,O/d,dirs_exist_ok=True)
shutil.copytree(R/'cli-profile-guidance-main8269-check-cycle-20261018',O/'actual-receipts',dirs_exist_ok=True)
shutil.copy2(R/'cli-profile-guidance-main8269-source-integration-20261018/SOURCE-INTEGRATION.json',O/'SOURCE-INTEGRATION.json')
shutil.copy2(R/'run_cli_profile_guidance_check_20261018_main8269.py',O/'ACTUAL-CHECK-DRIVER.py')
integration=readj(O/'SOURCE-INTEGRATION.json');assert sha((O/'ACTUAL-CHECK-DRIVER.py').read_bytes())==integration['driverSha256']
diff=git('diff',B,H,'--',*changed);(O/'ACTUAL-THREE-PATH.patch').write_bytes(diff);assert sha(diff)==integration['candidateDiffSha256'];assert integration['wholeCandidateDiffByteIdentical']
files=[]
for p in [v['path'] for v in oldpins['files']]+['crates/mori/src/commands/check_source.rs']:
 b=obj(p);dest=O/'source'/p;dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(b)
 files.append({'path':p,'commit':H,'gitBlob':git('rev-parse',H+':'+p).decode().strip(),'sha256':sha(b),'bytes':len(b),'changed':p in changed})
 if p in changed:assert b==(P/'source'/p).read_bytes()
by={f['path']:f for f in files}
assert obj('crates/mori/src/json.rs')==(P/'source/crates/mori/src/json.rs').read_bytes()
newpins={'head':H,'tree':T,'base':B,'changedPaths':changed,'files':files,'absentAtCandidate':[],'preparationContextRecords':37,'presentContextRecords':37,'historicalPreparation':'source-preparation/ unchanged main79 preparation; absence claim is historical, superseded by actual present helper below'}
dump(O/'CURRENT-SOURCE-PINS.json',newpins)
previous=readj(P/'QUOTE-HASH-COVERAGE.json');cov=[];sections=[];quotes=[]
for old in previous['coverage']:
 p=old['path'];f=by[p];r=dict(old);r.update(f);r.pop('sourcePrepBaseSha256',None);r.pop('note',None)
 b=(O/'source'/p).read_bytes();s=b.decode();lines=s.splitlines(keepends=True)
 if p in ['crates/mori/src/commands/check.rs','crates/mori/src/commands/check_source.rs','crates/mori/src/commands/mod.rs']:
  r['mode']='complete-context-body';r['lineRanges']=[[1,len(lines)]];r['id']='C40' if p.endswith('check_source.rs') else r['id']
 if r['mode'].startswith('complete-'):
  q=s;r['lineRanges']=[[1,len(lines)]]
 elif r['mode']=='exact-line-ranges':q=''.join('LINES '+str(lo)+'–'+str(hi)+'\n'+''.join(lines[lo-1:hi]) for lo,hi in r['lineRanges'])
 elif r['mode']=='json-pointer-values':
  x=json.loads(s);q=json.dumps({p:ptr(x,p) for p in r['jsonPointers']},separators=(',',':'))+'\n'
 else:raise ValueError(r)
 quotes.append(q);r['physicalBody']='source/'+p;r['quoteSha256']=sha(q.encode());r['quoteBytes']=len(q.encode());r['readerCoverageClaim']='Quoted full/ranged/pointer content only; physical full body available, never presumed read.'
 cov.append(r)
 sections.append('### '+r['id']+' '+p+' — '+r['mode']+'\n\n'+('Whole SHA256 '+f['sha256']+'.\n\n' if p in changed else '')+'````text\n'+q+('' if q.endswith('\n') else '\n')+'````\n')
coverage={'contextPreparationRecords':37,'presentPreparationContextRecords':37,'changedBodiesMandatoryComplete':changed,'allActualPhysicalBodies':len(files),'modelReadDiscipline':'Only declared quotations or extra bodies actually read count as reviewer coverage. Large JSON full bodies stored physically are not implied read. JSON pointers are semantic values, not whole-output or wire goldens.','coverage':cov}
dump(O/'QUOTE-HASH-COVERAGE.json',coverage)
source='## Exact changed bodies and direct consumer context\n\nAll three changed bodies are complete. All 37 prepared-context paths are now present. Actual check.rs, shared check_source.rs and commands/mod.rs are complete below, refreshed at f297 after PR227. The CheckReport definitions are byte-identical to the historical d7a source. Other contexts retain exact quote ranges/pointers, with fresh whole-body Git/SHA pins; all 40 full physical bodies are available. No whole model read is inferred from file presence.\n\n'+'\n'.join(sections)
(O/'SOURCE-CONTEXT.md').write_text(source)
contract=readj(P/'ORDINARY-REVIEW-CONTRACT.json');contract['scope']=contract['scope'].replace('main79','actual main8269 integration f297')
contract['sourcePredicates'][2]='Guide command spellings, profiles, flags, reported shapes, links and caveats agree with actual current consumers. PR227 now routes check.rs through check_source::dispatch; inspect both complete bodies and module mapping. Applicable CheckReport is unchanged.'
contract['history']='Historical d7a packet and receipts remain immutable. No historical approval/result is transferred to f297; this packet uses new exact f297 terminal receipts.'
dump(O/'ORDINARY-REVIEW-CONTRACT.json',contract)
plan=readj(O/'actual-receipts/CHECK-PLAN.json'); final=readj(O/'actual-receipts/FINAL-RESULT.json'); xp=readj(O/'actual-receipts/SOURCE-PINS.json')
assert sha((O/'source-preparation/MANIFEST.json').read_bytes())==plan['sourceManifestSha256']
assert plan['head']==final['head']==xp['head']==H and plan['tree']==final['tree']==xp['tree']==T and plan['base']==B
assert final['allSixChecksCompleted'] and not final['existsAfter']
execpins={v['path']:v['sha256'] for v in xp['files']}; matched=[]
for f in files:
 if f['path'] in execpins:assert execpins[f['path']]==f['sha256'];matched.append(f['path'])
for result in final['results']:
 n=result['name']; assert result['exitCode']==0 and result['ownedSessionRemaining']==[]
 assert readj(O/'actual-receipts'/n/'RESULT.json')=={k:v for k,v in result.items() if k!='name'}
 for stream in ['STDOUT','STDERR']:assert sha((O/'actual-receipts'/n/(stream+'.log')).read_bytes())==result[stream.lower()+'Sha256']
 start=readj(O/'actual-receipts'/n/'START.json');assert start['head']==H and start['tree']==T
 if n.startswith('shell-'):
  assert (O/'actual-receipts'/n/'STDOUT.log').stat().st_size==(O/'actual-receipts'/n/'STDERR.log').stat().st_size==0
  shell=O/'source-preparation/doc-shell'/('0'+n[-1]+'.sh');assert shell.read_bytes()==pathlib.Path(start['argv'][-1]).read_bytes()
  assert shell.read_text().strip() in obj('docs/NATIVE-CORE.md').decode()
stdout=(O/'actual-receipts/clap-help-test/STDOUT.log').read_text();assert '1 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out' in stdout
summary={'format':'cli-profile-guidance-actual-result-summary/1','source':{'head':H,'tree':T,'base':B,'changedPaths':changed},'rootReportedDriver':{'terminalExitCode':0,'sourceCleanUnchanged':True,'evidence':'Root collected tool77398 terminal0, exact clean unchanged source; full driver copied, not rerun by preparer.'},'observedReceipts':{'clapPassed':1,'clapFailed':0,'clapFiltered':7,'clapWallSeconds':final['results'][0]['wallSeconds'],'shellSyntaxPasses':5,'shellStdoutStderrBytes':0,'allSixChecksCompleted':True,'allOwnedSessionArraysEmpty':True,'ownedRootExistsAfter':False,'sourcePinsMetadataEntries':len(xp['files']),'actualQuotedSourceBodiesMatchedExecutionPins':len(matched),'matchedPaths':matched},'limits':readj(P/'ACTUAL-RESULT-SUMMARY.json')['limits']}
dump(O/'ACTUAL-RESULT-SUMMARY.json',summary)
subject={'format':'cli-profile-guidance-main8269-source-result-subject/1','head':H,'tree':T,'base':B,'changedPaths':changed,'changedPins':[f for f in files if f['changed']],'actualRuntimeScope':'1selectedClap metadata PASS/7filtered;5bash-nPASS; no documentation product/native/SDK execution'}
for name,key in [('CURRENT-SOURCE-PINS.json','contextPinsSha256'),('QUOTE-HASH-COVERAGE.json','quoteCoverageSha256'),('ORDINARY-REVIEW-CONTRACT.json','reviewContractSha256'),('actual-receipts/CHECK-PLAN.json','checkPlanSha256'),('actual-receipts/SOURCE-PINS.json','rootSourcePinsSha256'),('actual-receipts/FINAL-RESULT.json','finalResultSha256'),('ACTUAL-RESULT-SUMMARY.json','actualSummarySha256'),('ACTUAL-CHECK-DRIVER.py','checkDriverSha256'),('SOURCE-INTEGRATION.json','integrationSha256')]:subject[key]=sha((O/name).read_bytes())
dump(O/'SUBJECT.json',subject);subjectsha=sha((O/'SUBJECT.json').read_bytes());token='doctorwatson-cli-profile-guidance-main8269-source-result-ordinary-20261018-a1-whole-review'
schema=readj(P/'OUTPUT-SCHEMA.json');schema['properties']['reviewToken']['const']=token;schema['properties']['reviewedSubjectSha256']['const']=subjectsha;dump(O/'OUTPUT-SCHEMA.json',schema)
primary='# Fresh exact ordinary CLI profile-guidance source/result review\n\nHead '+H+'; tree '+T+'; base '+B+'. Subject SHA256 '+subjectsha+'; review token `'+token+'`. Full actual three changed bodies below. Ordinary source and result review only; no approval supplied, forced empty findings, security/native admission or parent closure. Historical packet remains unchanged.\n\n````json\n'+json.dumps(contract,separators=(',',':'))+'\n````\n\n'+source
primary+='\n## New exact terminal receipts\n\nIntake occurred only after root reported tool77398 terminal0. Source/context hash matching here is transport verification, not extra execution.\n\n'
primary+='New actual evidence: one selected Clap pass in '+str(final['results'][0]['wallSeconds'])+' seconds, zero failed/ignored/measured, seven filtered. Five bash -n checks passed with empty streams. All six owned-session arrays are empty; final existsAfter=false. Exact current CHECK-PLAN/FINAL-RESULT and per-check receipts are physical; no historical runtime transferred.\n\nActual selected argv:\n\n````json\n'+json.dumps(readj(O/'actual-receipts/clap-help-test/START.json')['argv'])+'\n````\n\n'
primary+='Complete actual selected Clap stdout:\n\n````text\n'+stdout+'````\n\nClap stderr is nonempty build progress; complete physical stream SHA256 '+final['results'][0]['stderrSha256']+'. Final two lines:\n\n````text\n'+''.join((O/'actual-receipts/clap-help-test/STDERR.log').read_text().splitlines(keepends=True)[-2:])+'````\n\n'
primary+='All CHECK-PLAN/SOURCE-PINS/FINAL-RESULT and all six START/CHILD/RESULT/full stdout/stderr exports are physical under actual-receipts/. The full actual driver is ACTUAL-CHECK-DRIVER.py. Root pin metadata entries are not claimed as bodies read. Source-preparation/ preserves earlier source-derived expectations and five exact shell inputs unchanged, including historical UNRUN/main79 labels. Only current selected metadata test and shell syntax have new runtime evidence. No documented product command, Node beta, extraction script, install, compiler/SDK/native/proof/ledger example was run. Assess command/context claims from actual sources; identify any unsatisfied predicate. Report actual quotation ranges/JSON pointers and any extra bodies read, plus unread/insufficient scope. Full source bodies are available but not presumed read. All U0–U7/M1–M17/MC/SP/G/148requirements/292scenarios/20families/finance/composition/native/target and security-owner limits remain.\n'
(O/'PRIMARY.md').write_text(primary)
intro='Fresh nonauthor ordinary source and actual-result review. Read PRIMARY and needed physical context. All three changed bodies and complete current checker wrapper/helper/module are inline; other direct/doc contexts have declared full/ranged/pointer coverage. Separate sourceAssessment/actualResultAssessment and actual reads/gaps. No execution, writes, provider calls, security/native judgment or parent closure. Return one complete OUTPUT-SCHEMA object; substantive extras and unbounded reasons allowed, no forced verdict or empty findings.\n\n'
(O/'PROMPT.md').write_text(intro+primary)
# Verify exact quote presence and all copied public receipt bytes.
for c in cov:
 b=(O/'source'/c['path']).read_bytes();assert sha(b)==c['sha256']
 assert quotes[cov.index(c)] in primary
for p in (O/'actual-receipts').rglob('*'):
 if p.is_file():assert p.read_bytes()==(R/'cli-profile-guidance-main8269-check-cycle-20261018'/p.relative_to(O/'actual-receipts')).read_bytes()
try:
 import jsonschema
 jsonschema.Draft202012Validator.check_schema(schema)
except ImportError:raise RuntimeError('schema validator missing')
verification={'status':'PASS_PUBLIC_SOURCE_QUOTE_RECEIPT_SCHEMA_HASH_TRANSPORT_ONLY','changedBodiesCompleteInline':3,'actualSourceBodies':len(files),'contextPreparationRecords':37,'presentContextRecords':37,'absentContextRecords':0,'coverageRows':len(cov),'actualCurrentCheckerHelperModuleComplete':True,'CheckReportSourceByteIdenticalToHistorical':True,'original3GuidanceBodiesByteIdentical':True,'wholeCandidateDiffByteIdentical':True,'newReceiptsOnly':True,'actualSourceResult':'1selectedClapPASS/7filtered;5bash-nPASS;noDocProductExecution','primaryBytes':len(primary.encode()),'promptBytes':(O/'PROMPT.md').stat().st_size,'providerDispatches':0,'newRuntimeExecutionByPreparer':0,'GitOrSharedCheckoutMutation':False,'resultIntakeAfterRootTerminalMessage':True,'schema':'VALID_JSON_SCHEMA; no max string/findings lengths or forced-empty fields','historicalPacketUnchanged':'Old directory not written; original artifacts preserved.'}
dump(O/'STATIC-VERIFICATION.json',verification)
(O/'README.md').write_text('Fresh ordinary source/result review input for f297/main8269.\n\nPROMPT.md is the complete reviewer primary. SUBJECT.json pins exact current source and new receipts. OUTPUT-SCHEMA separates source/result judgments and honest coverage; no verdict is supplied. All 3 changed bodies, complete actual current check.rs/check_source.rs/mod.rs, and declared direct consumer/doc quotations are inline. All 40 public full source bodies are physical. Earlier main79 preparation labels are historical; new current pins supersede the former absent-helper context.\n\nNew root execution only: 1 selected Clap metadata pass in '+str(final['results'][0]['wallSeconds'])+'s,7filtered;5bash-n pass/empty streams; owned sessions empty/temp root deletion. All receipts copied unchanged after root terminal0. No doc product examples or native/SDK/proofs ran. Previous results/approvals are not transferred. Quote/pin/schema/hash transport only performed here; no provider/product runtime/Git mutation.\n\nInput permits substantive findings and insufficient context. All original limits and exits remain; no security/native or financial oracle judgment. See STATIC-VERIFICATION and QUOTE-HASH-COVERAGE.\n')
manifest={'format':'cli-profile-guidance-main8269-review-packet/1','head':H,'tree':T,'base':B,'files':[]}
for p in sorted(O.rglob('*')):
 if p.is_file() and p.name not in ['MANIFEST.json','FREEZE.sha256']:manifest['files'].append({'path':str(p.relative_to(O)),'bytes':p.stat().st_size,'sha256':sha(p.read_bytes())})
dump(O/'MANIFEST.json',manifest)
freeze=''.join(sha(p.read_bytes())+'  '+str(p.relative_to(O))+'\n' for p in sorted(O.rglob('*')) if p.is_file() and p.name!='FREEZE.sha256');(O/'FREEZE.sha256').write_text(freeze)
print(json.dumps({'output':str(O),'subjectSha256':subjectsha,'promptBytes':(O/'PROMPT.md').stat().st_size,'primaryBytes':len(primary.encode()),'schemaSha256':sha((O/'OUTPUT-SCHEMA.json').read_bytes()),'manifestSha256':sha((O/'MANIFEST.json').read_bytes()),'freezeSha256':sha((O/'FREEZE.sha256').read_bytes()),'sourceBodies':len(files),'execPinMatches':len(matched)},indent=2))
