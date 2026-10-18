from pathlib import Path
import subprocess,datetime,time,json,hashlib,tempfile,shutil,os,signal
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017');K=Path('/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018');C=R/'source6-capped-remittance-node-check-cycle';O=R/'source6-capped-remittance-independent-oracle-20261018'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def write(p,x):p.write_text(json.dumps(x,indent=2)+'\n')
def git(*a):return subprocess.check_output(['git',*a],cwd=K).decode().strip()
source=json.loads((C/'CURRENT-SOURCE.json').read_text());head=source['head'];tree=source['tree'];assert git('rev-parse','HEAD')==head and not git('status','--porcelain=v1')
assert sha(O/'FREEZE.sha256')=='4a859749e91cc31cce74a1e9220e91235c9263a50e5dee1d84949e158885ba79'
lines=(O/'FREEZE.sha256').read_text().splitlines();assert len(lines)==23
for line in lines:
 h,name=line.split(maxsplit=1);name=name.lstrip('*');assert sha(O/name)==h,name
for row in source['files']:assert sha(K/row['path'])==row['sha256']
prior=json.loads((R/'core2-partial-delivery-check-cycle/SOURCE-PINS.json').read_text())['files']
ts_paths=set(subprocess.check_output(['git','ls-tree','-r','--name-only',head,'--','experiments/moriarty-language/src','experiments/moriarty-language/package.json'],cwd=K).decode().splitlines())
paths=ts_paths|{r['path'] for r in prior}|{r['path'] for r in source['files']}|set(json.loads((R/'source6-public-consumer-check-cycle/CURRENT-SOURCE.json').read_text())['changedPaths'])|{'crates/mori/src/commands/init.rs','crates/mori/tests/init.rs'}
pins={}
for path in sorted(paths):
 expected_bytes=subprocess.check_output(['git','show',head+':'+path],cwd=K)
 actual_bytes=(K/path).read_bytes();assert actual_bytes==expected_bytes,path
 pins[path]={'path':path,'sha256':hashlib.sha256(expected_bytes).hexdigest(),'bytes':len(expected_bytes),'gitBlob':git('rev-parse',head+':'+path)}
write(C/'SOURCE-PINS.json',{'sourceCommit':head,'sourceTree':tree,'files':list(pins.values())})
root=Path(tempfile.mkdtemp(prefix='doctorwatson-source6-min-node-'));(root/'cwd').mkdir();(root/'tmp').mkdir();(root/'cargo-home').mkdir()
for name in ['registry','git']:(root/'cargo-home'/name).symlink_to(Path('/home/charl/.cargo')/name,target_is_directory=True)
toolchain=Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin');tools=[toolchain/'cargo',toolchain/'rustc',toolchain/'rustdoc',Path('/usr/bin/timeout'),Path('/usr/local/bin/node')];toolpins=[{'path':str(p),'resolved':str(p.resolve()),'sha256':sha(p.resolve())} for p in tools]
configs=[]
for directory in list(K.parents)+[K]+list((root/'cwd').parents)+[root/'cwd']:
 for filename in ['config','config.toml']:
  p=directory/'.cargo'/filename
  if str(p) not in configs:assert not p.exists(),str(p);configs.append(str(p))
env={'PATH':str(toolchain)+':/usr/bin:/bin','RUSTC':str(toolchain/'rustc'),'RUSTDOC':str(toolchain/'rustdoc'),'CARGO_HOME':str(root/'cargo-home'),'CARGO_TARGET_DIR':str(root/'target'),'CARGO_NET_OFFLINE':'true','CARGO_INCREMENTAL':'0','CARGO_CACHE_AUTO_CLEAN_FREQUENCY':'never','GIT_CONFIG_NOSYSTEM':'1','GIT_CONFIG_GLOBAL':'/dev/null','GIT_CONFIG_SYSTEM':'/dev/null','LANG':'C.UTF-8','TMPDIR':str(root/'tmp')}
steps=[('build-public-rust-binary','mori',''),('node-capped-public-consumer','mori','')]
plan={'preparedAt':now(),'sourceCommit':head,'sourceTree':tree,'authorization':'Existing ordinary source/tooling tests; no SDK/native/model/key/proof/ledger execution. No historical cap reset.','sourceAudit':'Root inspected ordinary test/source/input and existing consumer; fresh AGY full source/result review before merge','oracleFreezeSha256':sha(O/'FREEZE.sha256'),'oracleMembersVerified':23,'independentScope':'Independent source values, complete semantic Source6 statement vectors and conditional financial frames for two min branches, fee0; prior Source6 exposure disclosed. Raw expected presentation authored separately; no whole Program/Artifact/hash/work/gate qualification oracle.','ownedTemporaryRoot':str(root),'publicCacheSymlinks':['registry','git'],'baseEnvironment':env,'toolPins':toolpins,'absentAncestorConfigs':configs,'attemptsPerStep':1,'timeoutSecondsPerStep':90,'killGraceSeconds':5,'sharedColdTarget':'One cold native-OFF Source6 binary build and two new actual TypeScript consumer cases; previous same-action Rusttext test not rerun. No install/fetch/SDK.','RAMDiskAggregateCaps':None,'originalExitsRemainOpen':True}
write(C/'CHECK-PLAN.json',plan);results=[]
try:
 for name,package,target in steps:
  d=C/name;d.mkdir();cmd=([str(toolchain/'cargo'),'build','--manifest-path',str(K/'Cargo.toml'),'--frozen','-j1','-p','mori','--no-default-features','--features','source6-expansion','--bin','mori'] if name=='build-public-rust-binary' else ['/usr/local/bin/node','--test',str(K/'experiments/moriarty-language/regressions/rust-source6-capped-remittance.mjs')]);child_env=dict(env)
  if name=='node-capped-public-consumer':
   child_env.update(MORI_RUST_BIN=str(root/'target/debug/mori'),MORI_SOURCE6_OBSERVATIONS=str(C/'node-observations'));write(d/'BINARY-IDENTITY.json',{'path':str(root/'target/debug/mori'),'sha256':sha(root/'target/debug/mori')})
  argv=['/usr/bin/timeout','--signal=TERM','--kill-after=5s','90s',*cmd]
  for p,row in pins.items():assert sha(K/p)==row['sha256'],p
  write(d/'START.json',{'startedAt':now(),'sourceCommit':head,'sourceTree':tree,'sourceStatus':git('status','--porcelain=v1'),'argv':argv,'cwd':str(root/'cwd'),'environment':child_env,'sourcePinsVerified':len(pins),'attempt':1,'attemptLimit':1})
  start=time.monotonic()
  with (d/'STDOUT.log').open('wb') as out,(d/'STDERR.log').open('wb') as err:
   child=subprocess.Popen(argv,cwd=root/'cwd',env=child_env,stdout=out,stderr=err,start_new_session=True);sid=child.pid;write(d/'CHILD.json',{'pid':sid,'session':sid});code=child.wait()
  remaining=[]
  for line in subprocess.check_output(['ps','-eo','pid=,sid=']).decode().splitlines():
   pid,s=line.split()
   if int(s)==sid:remaining.append(int(pid))
  write(d/'PROCESS-CLEANUP.json',{'ownedSession':sid,'remainingSessionMembers':remaining,'scope':'Owned SID census only; no syscall/detached-process exclusion.'});assert not remaining,remaining
  result={'finishedAt':now(),'exitCode':code,'wallSeconds':time.monotonic()-start,'sourceCommitAfter':git('rev-parse','HEAD'),'sourceTreeAfter':git('rev-parse','HEAD^{tree}'),'sourceStatusAfter':git('status','--porcelain=v1'),'stdoutSha256':sha(d/'STDOUT.log'),'stderrSha256':sha(d/'STDERR.log'),'attemptConsumed':True};write(d/'RESULT.json',result);results.append({'step':name,**result});print(json.dumps(results[-1]),flush=True)
  assert code==0 and result['sourceCommitAfter']==head and result['sourceTreeAfter']==tree and not result['sourceStatusAfter']
  for p,row in pins.items():assert sha(K/p)==row['sha256'],p
finally:
 shutil.rmtree(root);write(C/'CLEANUP.json',{'recordedAt':now(),'ownedTemporaryRoot':str(root),'existsAfter':root.exists(),'scope':'Only owned temporary root removed, public cache symlinks not traversed.'});write(C/'RESULT.json',{'recordedAt':now(),'sourceCommit':head,'sourceTree':tree,'steps':results,'temporaryRootExistsAfter':root.exists(),'allStepsCompleted':len(results)==2,'allExitZero':len(results)==2 and all(x['exitCode']==0 for x in results),'scope':'Real Rust output for both min branches through actual experimental Source6 parser/lowerer and provisional S0 consumer; complete independent semantic AST/effect/financial projections only. No wholeCoreProgram/Artifact/measurement/authentication/native/proof/ledger qualification.'})
