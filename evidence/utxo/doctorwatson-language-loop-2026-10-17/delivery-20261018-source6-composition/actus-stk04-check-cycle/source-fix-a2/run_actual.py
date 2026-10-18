from pathlib import Path
import subprocess,datetime,time,json,hashlib,tempfile,shutil,os,signal
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017');K=Path('/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018');C=R/'actus-stk04-check-cycle'/'source-fix-a2';O=R/'actus-stk04-independent-oracles'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def write(p,x):p.write_text(json.dumps(x,indent=2)+'\n')
def git(*a):return subprocess.check_output(['git',*a],cwd=K).decode().strip()
source=json.loads((C/'CURRENT-SOURCE.json').read_text());head=source['sourceCommit'];tree=source['sourceTree'];assert git('rev-parse','HEAD')==head and not git('status','--porcelain=v1')
assert sha(O/'FREEZE.sha256')=='3acc0f0064164026aecbf100bebd3c8da455510ae2d9ea83bb621065192ca3ab'
lines=(O/'FREEZE.sha256').read_text().splitlines();assert len(lines)==33
for line in lines:
 h,name=line.split(maxsplit=1);name=name.lstrip('*');assert sha(O/name)==h,name
for row in source['sourceFiles']:assert sha(K/row['path'])==row['sha256']
prior=json.loads((R/'core2-partial-delivery-check-cycle/SOURCE-PINS.json').read_text())['files']
paths={r['path'] for r in prior}|{r['path'] for r in source['sourceFiles']}|set(json.loads((R/'source6-public-consumer-check-cycle/CURRENT-SOURCE.json').read_text())['changedPaths'])|{'crates/mori/src/commands/init.rs','crates/mori/tests/init.rs'}
pins={}
for path in sorted(paths):
 expected_bytes=subprocess.check_output(['git','show',head+':'+path],cwd=K)
 actual_bytes=(K/path).read_bytes();assert actual_bytes==expected_bytes,path
 pins[path]={'path':path,'sha256':hashlib.sha256(expected_bytes).hexdigest(),'bytes':len(expected_bytes),'gitBlob':git('rev-parse',head+':'+path)}
write(C/'SOURCE-PINS.json',{'sourceCommit':head,'sourceTree':tree,'files':list(pins.values())})
root=Path(tempfile.mkdtemp(prefix='doctorwatson-actus-stk04-'));(root/'cwd').mkdir();(root/'tmp').mkdir();(root/'cargo-home').mkdir()
for name in ['registry','git']:(root/'cargo-home'/name).symlink_to(Path('/home/charl/.cargo')/name,target_is_directory=True)
toolchain=Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin');tools=[toolchain/'cargo',toolchain/'rustc',toolchain/'rustdoc',Path('/usr/bin/timeout')];toolpins=[{'path':str(p),'resolved':str(p.resolve()),'sha256':sha(p.resolve())} for p in tools]
configs=[]
for directory in list(K.parents)+[K]+list((root/'cwd').parents)+[root/'cwd']:
 for filename in ['config','config.toml']:
  p=directory/'.cargo'/filename
  if str(p) not in configs:assert not p.exists(),str(p);configs.append(str(p))
env={'PATH':str(toolchain)+':/usr/bin:/bin','RUSTC':str(toolchain/'rustc'),'RUSTDOC':str(toolchain/'rustdoc'),'CARGO_HOME':str(root/'cargo-home'),'CARGO_TARGET_DIR':str(root/'target'),'CARGO_NET_OFFLINE':'true','CARGO_INCREMENTAL':'0','CARGO_CACHE_AUTO_CLEAN_FREQUENCY':'never','GIT_CONFIG_NOSYSTEM':'1','GIT_CONFIG_GLOBAL':'/dev/null','GIT_CONFIG_SYSTEM':'/dev/null','LANG':'C.UTF-8','TMPDIR':str(root/'tmp')}
steps=[('stk04-independent-semantic-trace','mori-core','financial_pair')]
plan={'preparedAt':now(),'sourceCommit':head,'sourceTree':tree,'authorization':'Existing ordinary source/tooling tests; no SDK/native/model/key/proof/ledger execution. No historical cap reset.','sourceAudit':'Root inspected ordinary test/source/input and existing consumer; fresh AGY full source/result review before merge','oracleFreezeSha256':sha(O/'FREEZE.sha256'),'oracleMembersVerified':33,'independentScope':'Independent source-derived complete semantic traces/snapshots/ordered effects/four ACTUS cashflow fields; numeric ExprIds/full wire/actualmeter/three ACTUS state mappings remain unasserted; prior related exposure declared.','ownedTemporaryRoot':str(root),'publicCacheSymlinks':['registry','git'],'baseEnvironment':env,'toolPins':toolpins,'absentAncestorConfigs':configs,'attemptsPerStep':1,'timeoutSecondsPerStep':90,'killGraceSeconds':5,'sharedColdTarget':'One newly selected stk04 test only; existing two passing financial_pair tests filtered out, no warmup/install/fetch.','RAMDiskAggregateCaps':None,'originalExitsRemainOpen':True}
write(C/'CHECK-PLAN.json',plan);results=[]
try:
 for name,package,target in steps:
  d=C/name;d.mkdir();cmd=[str(toolchain/'cargo'),'test','--manifest-path',str(K/'Cargo.toml'),'--frozen','-j1','-p',package,'--no-default-features','--test',target,'actus_stk04_matches_independent_core_trace_and_four_cashflow_fields','--','--exact','--test-threads=1'];argv=['/usr/bin/timeout','--signal=TERM','--kill-after=5s','90s',*cmd]
  for p,row in pins.items():assert sha(K/p)==row['sha256'],p
  write(d/'START.json',{'startedAt':now(),'sourceCommit':head,'sourceTree':tree,'sourceStatus':git('status','--porcelain=v1'),'argv':argv,'cwd':str(root/'cwd'),'environment':env,'sourcePinsVerified':len(pins),'attempt':1,'attemptLimit':1})
  start=time.monotonic()
  with (d/'STDOUT.log').open('wb') as out,(d/'STDERR.log').open('wb') as err:
   child=subprocess.Popen(argv,cwd=root/'cwd',env=env,stdout=out,stderr=err,start_new_session=True);sid=child.pid;write(d/'CHILD.json',{'pid':sid,'session':sid});code=child.wait()
  remaining=[]
  for line in subprocess.check_output(['ps','-eo','pid=,sid=']).decode().splitlines():
   pid,s=line.split()
   if int(s)==sid:remaining.append(int(pid))
  write(d/'PROCESS-CLEANUP.json',{'ownedSession':sid,'remainingSessionMembers':remaining,'scope':'Owned SID census only; no syscall/detached-process exclusion.'});assert not remaining,remaining
  result={'finishedAt':now(),'exitCode':code,'wallSeconds':time.monotonic()-start,'sourceCommitAfter':git('rev-parse','HEAD'),'sourceTreeAfter':git('rev-parse','HEAD^{tree}'),'sourceStatusAfter':git('status','--porcelain=v1'),'stdoutSha256':sha(d/'STDOUT.log'),'stderrSha256':sha(d/'STDERR.log'),'attemptConsumed':True};write(d/'RESULT.json',result);results.append({'step':name,**result});print(json.dumps(results[-1]),flush=True)
  assert code==0 and result['sourceCommitAfter']==head and result['sourceTreeAfter']==tree and not result['sourceStatusAfter']
  for p,row in pins.items():assert sha(K/p)==row['sha256'],p
finally:
 shutil.rmtree(root);write(C/'CLEANUP.json',{'recordedAt':now(),'ownedTemporaryRoot':str(root),'existsAfter':root.exists(),'scope':'Only owned temporary root removed, public cache symlinks not traversed.'});write(C/'RESULT.json',{'recordedAt':now(),'sourceCommit':head,'sourceTree':tree,'steps':results,'temporaryRootExistsAfter':root.exists(),'allStepsCompleted':len(results)==1,'allExitZero':len(results)==1 and all(x['exitCode']==0 for x in results),'scope':'One STK04 Core semantic test/two real public Core calls/14complete typed step values and snapshots/eight ordered effects/four known cashflow fields; no full7-field ACTUS fixture/native/proof/ledger qualification.'})
