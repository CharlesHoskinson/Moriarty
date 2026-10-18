from pathlib import Path
import subprocess,datetime,time,json,hashlib,shutil
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017');K=Path('/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018');C=R/'shared-check-production-check-cycle';plan=json.loads((C/'BASELINE-CHECK-PLAN.json').read_text());root=Path(plan['ownedTemporaryRoot']);env=plan['environment'];source=json.loads((C/'CURRENT-SOURCE.json').read_text());head=source['head'];tree=source['tree'];tc=Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def write(p,x):p.write_text(json.dumps(x,indent=2)+'\n')
def git(*a):return subprocess.check_output(['git',*a],cwd=K).decode().strip()
assert head=='823ce081c00786a0c55b92af81009f9e7c5e6a16' and git('rev-parse','HEAD')==head and not git('status','--porcelain=v1');assert root.is_dir()
old=json.loads((C/'BASELINE-SOURCE-PINS.json').read_text());changed={r['path'] for r in source['files']};verified=0
for row in old['files']:
 if row['path'] not in changed and (K/row['path']).exists():assert sha(K/row['path'])==row['sha256'],row['path'];verified+=1
for row in source['files']:assert sha(K/row['path'])==row['sha256']
write(C/'CHANGED-CHECK-PLAN.json',{'recordedAt':now(),'head':head,'tree':tree,'environment':env,'ownedTemporaryRoot':str(root),'changedSourcePinsVerified':len(source['files']),'baselineUnaffectedPinsVerified':verified,'inputFreezeSha256':sha(C/'INPUT-CASES-FROZEN.json'),'steps':'One8-test publicbinaryintegration target plus eight directcheckJSON compares toactualbaseline. Existing ownedtarget reused, no dependencyinstall/fetch/defaultnative.','bound':'90s+5s test command;10s+5s eachdirectcheck','scope':'Actual source-only CLI/checker and futureMCP prerequisite; no MCPserver/native/SDK/financial/proof/ledgeracceptance.'})
def run(name,cmd,bound,cwd):
 d=C/name;d.mkdir(parents=True);argv=['/usr/bin/timeout','--signal=TERM','--kill-after=5s',str(bound)+'s',*cmd];write(d/'START.json',{'startedAt':now(),'argv':argv,'cwd':str(cwd),'environment':env,'head':head,'tree':tree,'attempt':1});start=time.monotonic()
 with (d/'STDOUT.log').open('wb') as out,(d/'STDERR.log').open('wb') as err:
  child=subprocess.Popen(argv,cwd=cwd,env=env,stdin=subprocess.DEVNULL,stdout=out,stderr=err,start_new_session=True);write(d/'CHILD.json',{'pid':child.pid,'session':child.pid});code=child.wait()
 remaining=[int(p) for p,s in (line.split() for line in subprocess.check_output(['ps','-eo','pid=,sid=']).decode().splitlines()) if int(s)==child.pid];assert not remaining;result={'finishedAt':now(),'exitCode':code,'wallSeconds':time.monotonic()-start,'stdoutSha256':sha(d/'STDOUT.log'),'stderrSha256':sha(d/'STDERR.log'),'ownedSessionRemaining':remaining};write(d/'RESULT.json',result);return result,d
observations=[];tests=None
try:
 tests,d=run('changed-public-tests',[str(tc/'cargo'),'test','--manifest-path',str(K/'Cargo.toml'),'--frozen','-j1','-p','mori','--no-default-features','--test','check_source_dispatch','--','--test-threads=1'],90,root/'cwd');print(json.dumps({'tests':tests}),flush=True);assert tests['exitCode']==0 and '8 passed; 0 failed' in (d/'STDOUT.log').read_text(),tests
 cases=json.loads((C/'INPUT-CASES-FROZEN.json').read_text())
 for case in cases:
  cwd=root/'cwd'/case['name'];assert (cwd/case['file']).read_text()==case['source'];result,d=run('changed-cases/'+case['name'],[str(root/'target/debug/mori'),'--color','never','check',case['file'],'--json'],10,cwd);baseline=C/'baseline-cases'/case['name'];same=(d/'STDOUT.log').read_bytes()==(baseline/'STDOUT.log').read_bytes();expected=json.loads((baseline/'RESULT.json').read_text())['exitCode'];observations.append({'name':case['name'],**result,'sameWholeStdoutBytesAsActualBaseline':same,'sameExitAsActualBaseline':result['exitCode']==expected,'stderrEmpty':not (d/'STDERR.log').read_bytes()});assert same and result['exitCode']==expected and not (d/'STDERR.log').read_bytes(),observations[-1]
 assert git('rev-parse','HEAD')==head and git('rev-parse','HEAD^{tree}')==tree and not git('status','--porcelain=v1')
 for row in source['files']:assert sha(K/row['path'])==row['sha256']
 print(json.dumps({'directCases':len(observations),'allWholeBytesAndExitsEqual':True}),flush=True)
finally:
 shutil.rmtree(root);write(C/'FINAL-RESULT.json',{'finishedAt':now(),'head':head,'tree':tree,'tests':tests,'directObservations':observations,'allEightDirectCasesCompleted':len(observations)==8,'ownedTemporaryRoot':str(root),'ownedRootExistsAfter':root.exists(),'scope':'One8-testpublicCLI suite (eachJSON+quiethuman),8directwholebaseline comparisons. No MCPserver/SDK/native/proof/ledger/full-languageclosure.'});write(C/'CLEANUP.json',{'recordedAt':now(),'root':str(root),'existsAfter':root.exists(),'scope':'Only ownedtemporaryroot removed, cache symlinks nottraversed.'})
