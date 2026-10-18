from pathlib import Path
import subprocess,datetime,time,json,hashlib,tempfile,shutil
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017');K=Path('/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018');C=R/'public-capabilities-check-cycle/actual-check-a1';P=R/'capabilities-real-consumer-check-plan-20261018';C.mkdir()
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def write(p,x):p.write_text(json.dumps(x,indent=2)+'\n')
def git(*a):return subprocess.check_output(['git',*a],cwd=K).decode().strip()
source=json.loads((C.parent/'CURRENT-SOURCE.json').read_text());head=source['head'];tree=source['tree'];assert head=='ff4f718d8b1d9deccf40ff7597b6afbb1f1ccf75' and git('rev-parse','HEAD')==head and not git('status','--porcelain=v1')
paths=subprocess.check_output(['git','ls-tree','-r','--name-only',head,'crates','Cargo.toml','Cargo.lock','rust-toolchain.toml'],cwd=K).decode().splitlines();pins={}
for path in paths:
 data=subprocess.check_output(['git','show',head+':'+path],cwd=K);assert (K/path).read_bytes()==data,path;pins[path]={'path':path,'sha256':hashlib.sha256(data).hexdigest(),'bytes':len(data)}
write(C/'SOURCE-PINS.json',{'head':head,'tree':tree,'files':list(pins.values())})
expectedPins=[]
for name in ['no-default-features','ordinary-optins']:
 p=P/'specified-expected'/f'{name}.json';json.loads(p.read_text());expectedPins.append({'path':str(p),'sha256':sha(p),'provenance':'Author-specified complete public metadata contract, frozen before actual check. No monetary/work execution or independently derived financial claim.'})
root=Path(tempfile.mkdtemp(prefix='doctorwatson-capabilities-'));[(root/n).mkdir() for n in ['cwd','tmp','cargo-home']]
for name in ['registry','git']:(root/'cargo-home'/name).symlink_to(Path('/home/charl/.cargo')/name,target_is_directory=True)
toolchain=Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin');tools=[toolchain/'cargo',toolchain/'rustc',toolchain/'rustdoc',Path('/usr/bin/timeout')];toolpins=[{'path':str(p),'sha256':sha(p.resolve())} for p in tools]
configs=[]
for directory in [K,*K.parents,root/'cwd',*(root/'cwd').parents]:
 for filename in ['config','config.toml']:
  p=directory/'.cargo'/filename
  if str(p) not in configs:assert not p.exists(),str(p);configs.append(str(p))
env={'PATH':str(toolchain)+':/usr/bin:/bin','RUSTC':str(toolchain/'rustc'),'RUSTDOC':str(toolchain/'rustdoc'),'CARGO_HOME':str(root/'cargo-home'),'CARGO_TARGET_DIR':str(root/'target'),'CARGO_NET_OFFLINE':'true','CARGO_INCREMENTAL':'0','CARGO_CACHE_AUTO_CLEAN_FREQUENCY':'never','GIT_CONFIG_NOSYSTEM':'1','GIT_CONFIG_GLOBAL':'/dev/null','GIT_CONFIG_SYSTEM':'/dev/null','LANG':'C.UTF-8','TMPDIR':str(root/'tmp')}
base=[str(toolchain/'cargo'),'test','--manifest-path',str(K/'Cargo.toml'),'--frozen','-j1','-p','mori','--no-default-features']
steps=[('no-default-features',base+['--test','capabilities','--','--test-threads=1']),('ordinary-optins',base+['--features','source6-expansion,core2-disjoint-draft','--test','capabilities','--','--exact','public_query_distinguishes_profiles_build_consumers_and_unchecked_targets','--test-threads=1'])]
write(C/'CHECK-PLAN.json',{'recordedAt':now(),'head':head,'tree':tree,'sourcePins':len(pins),'toolPins':toolpins,'environment':env,'ownedTemporaryRoot':str(root),'absentAncestorConfigs':configs,'expectedContractPins':expectedPins,'steps':[{'name':n,'argv':a,'wallSeconds':90,'killGraceSeconds':5} for n,a in steps],'actualQueryPerFeature':{'argv':[str(root/'target/debug/mori'),'--color','never','capabilities','--json'],'wallSeconds':10,'killGraceSeconds':5},'scope':'Ordinary reporting and source-only check/inspect. No evaluator financial transaction, compiler, native SDK/VM/proof/key/wallet/ledger invocation; no prior cap reset.','sourceRead':'Root read whole4paths, actual dispatcher/check/inspect/print paths; v1/v2 defects preserved and v3 source corrections audited locally. Fresh independent review pending.'})
results=[]
try:
 for name,cmd in steps:
  assert git('rev-parse','HEAD')==head and not git('status','--porcelain=v1')
  for p,row in pins.items():assert sha(K/p)==row['sha256'],p
  d=C/name;d.mkdir();argv=['/usr/bin/timeout','--signal=TERM','--kill-after=5s','90s',*cmd];write(d/'START.json',{'startedAt':now(),'head':head,'tree':tree,'argv':argv,'cwd':str(root/'cwd'),'environment':env,'attempt':1})
  start=time.monotonic()
  with (d/'STDOUT.log').open('wb') as out,(d/'STDERR.log').open('wb') as err:
   child=subprocess.Popen(argv,cwd=root/'cwd',env=env,stdout=out,stderr=err,start_new_session=True);sid=child.pid;write(d/'CHILD.json',{'pid':sid,'session':sid});code=child.wait()
  remaining=[int(p) for p,s in (line.split() for line in subprocess.check_output(['ps','-eo','pid=,sid=']).decode().splitlines()) if int(s)==sid];write(d/'PROCESS-CLEANUP.json',{'ownedSession':sid,'remainingSessionMembers':remaining})
  result={'finishedAt':now(),'name':name,'exitCode':code,'wallSeconds':time.monotonic()-start,'stdoutSha256':sha(d/'STDOUT.log'),'stderrSha256':sha(d/'STDERR.log'),'headAfter':git('rev-parse','HEAD'),'treeAfter':git('rev-parse','HEAD^{tree}'),'statusAfter':git('status','--porcelain=v1')};write(d/'RESULT.json',result);results.append(result);print(json.dumps(result),flush=True)
  assert code==0 and not remaining and result['headAfter']==head and result['treeAfter']==tree and not result['statusAfter'],result
  assert ('2 passed; 0 failed' if name=='no-default-features' else '1 passed; 0 failed') in (d/'STDOUT.log').read_text()
  qargv=['/usr/bin/timeout','--signal=TERM','--kill-after=5s','10s',str(root/'target/debug/mori'),'--color','never','capabilities','--json'];q=subprocess.run(qargv,cwd=root/'cwd',env=env,capture_output=True);(d/'QUERY-STDOUT.json').write_bytes(q.stdout);(d/'QUERY-STDERR.log').write_bytes(q.stderr);expected=json.loads((P/'specified-expected'/f'{name}.json').read_text());actual=json.loads(q.stdout);equal=actual==expected;write(d/'QUERY-RESULT.json',{'argv':qargv,'exitCode':q.returncode,'stderrEmpty':not q.stderr,'completeJSONEqualsSpecifiedContract':equal,'stdoutSha256':sha(d/'QUERY-STDOUT.json')});assert q.returncode==0 and not q.stderr and equal
finally:
 shutil.rmtree(root);write(C/'CLEANUP.json',{'recordedAt':now(),'ownedTemporaryRoot':str(root),'existsAfter':root.exists(),'scope':'Only owned temp root removed; publiccache symlinks not traversed.'});write(C/'RESULT.json',{'recordedAt':now(),'head':head,'tree':tree,'steps':results,'allStepsCompleted':len(results)==2,'allExitZero':len(results)==2 and all(r['exitCode']==0 for r in results),'temporaryRootExistsAfter':root.exists(),'limits':'Selected normal reporting/source-only consumers; no proof/ledger/target acceptance.'})
