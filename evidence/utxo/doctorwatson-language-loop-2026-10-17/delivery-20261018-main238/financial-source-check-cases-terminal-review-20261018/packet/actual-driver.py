from pathlib import Path
import datetime,hashlib,json,os,shutil,subprocess,tempfile,time
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017')
K=Path('/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018')
P=R/'financial-check-cases-main79-20261018'
C=R/'financial-source-check-cases-check-cycle-20261018';C.mkdir()
HEAD='360f32ba5b6680788d1c0f1e7b68c2207a877d47'
TC=Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def write(name,x):
 p=C/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(x,indent=2)+'\n')
def git(*args):return subprocess.check_output(['git',*args],cwd=K).decode().strip()
assert git('rev-parse','HEAD')==HEAD and not git('status','--porcelain=v1')
tree=git('rev-parse','HEAD^{tree}')
assert tree=='66a7d788405966afe87f27e0879332652ee11125'
paths=subprocess.check_output(['git','ls-tree','-r','--name-only',HEAD,'crates','Cargo.toml','Cargo.lock','rust-toolchain.toml','docs/NATIVE-CORE.md'],cwd=K).decode().splitlines()
pins=[]
for path in paths:
 raw=subprocess.check_output(['git','show',HEAD+':'+path],cwd=K);assert(K/path).read_bytes()==raw,path
 pins.append({'path':path,'sha256':hashlib.sha256(raw).hexdigest()})
write('SOURCE-PINS.json',{'head':HEAD,'tree':tree,'files':pins})
manifest=json.loads((P/'MANIFEST.json').read_text())
root=Path(tempfile.mkdtemp(prefix='doctorwatson-cli-help-'))
for n in ['cwd','tmp','cargo-home']:(root/n).mkdir()
for n in ['registry','git']:(root/'cargo-home'/n).symlink_to(Path('/home/charl/.cargo')/n,target_is_directory=True)
env={'PATH':str(TC)+':/usr/bin:/bin','RUSTC':str(TC/'rustc'),'RUSTDOC':str(TC/'rustdoc'),'CARGO_HOME':str(root/'cargo-home'),'CARGO_TARGET_DIR':str(root/'target'),'CARGO_NET_OFFLINE':'true','CARGO_INCREMENTAL':'0','CARGO_CACHE_AUTO_CLEAN_FREQUENCY':'never','GIT_CONFIG_NOSYSTEM':'1','GIT_CONFIG_GLOBAL':'/dev/null','GIT_CONFIG_SYSTEM':'/dev/null','LANG':'C.UTF-8','TMPDIR':str(root/'tmp')}
configs=[]
for directory in [K,*K.parents,root/'cwd',*(root/'cwd').parents]:
 for n in ['config','config.toml']:
  p=directory/'.cargo'/n
  if str(p) not in configs:assert not p.exists(),str(p);configs.append(str(p))
write('CHECK-PLAN.json',{'recordedAt':now(),'head':HEAD,'tree':tree,'base':'8269b5b7ceeec5169c706aa31ea33f390d49c969','sourceManifestSha256':sha(P/'MANIFEST.json'),'environment':env,'ownedRoot':str(root),'absentAncestorCargoConfigs':configs,'toolPins':[{'path':str(p),'sha256':sha(p)} for p in [TC/'cargo',TC/'rustc',TC/'rustdoc',Path('/usr/bin/timeout'),Path('/usr/bin/bash')]],'scope':'Six selected normal financial source-check cases only, two public CLI invocations each. Eight unchanged prior cases not rerun; no financial evaluator/SDK/native/proof/key/wallet/ledger action.'})
def run(name,argv,bound):
 d=C/name;d.mkdir();argv=['/usr/bin/timeout','--signal=TERM','--kill-after=5s',str(bound)+'s',*argv]
 write(name+'/START.json',{'startedAt':now(),'argv':argv,'cwd':str(root/'cwd'),'head':HEAD,'tree':tree})
 start=time.monotonic()
 with(d/'STDOUT.log').open('wb')as out,(d/'STDERR.log').open('wb')as err:
  child=subprocess.Popen(argv,cwd=root/'cwd',env=env,stdin=subprocess.DEVNULL,stdout=out,stderr=err,start_new_session=True);write(name+'/CHILD.json',{'pid':child.pid,'session':child.pid});code=child.wait()
 remaining=[int(p)for p,s in(line.split()for line in subprocess.check_output(['ps','-eo','pid=,sid=']).decode().splitlines())if int(s)==child.pid]
 result={'finishedAt':now(),'exitCode':code,'wallSeconds':time.monotonic()-start,'stdoutSha256':sha(d/'STDOUT.log'),'stderrSha256':sha(d/'STDERR.log'),'ownedSessionRemaining':remaining};write(name+'/RESULT.json',result)
 assert not remaining;return result,d
results=[]
try:
 result,d=run('financial-source-check-tests',[str(TC/'cargo'),'test','--manifest-path',str(K/'Cargo.toml'),'--frozen','-j1','-p','mori','--no-default-features','--test','check_source_dispatch','financial_','--','--test-threads=1'],90)
 results.append({'name':'financial-source-check-tests',**result});print(json.dumps(results[-1]),flush=True)
 assert result['exitCode']==0
 text=(d/'STDOUT.log').read_text();assert '6 passed; 0 failed' in text and 'financial_core2_rebate ... ok' in text
 assert git('rev-parse','HEAD')==HEAD and not git('status','--porcelain=v1')and all(sha(K/p['path'])==p['sha256']for p in pins)
finally:
 shutil.rmtree(root)
 write('FINAL-RESULT.json',{'finishedAt':now(),'head':HEAD,'tree':tree,'results':results,'selectedSixCaseSuiteCompleted':len(results)==1,'ownedRoot':str(root),'existsAfter':root.exists(),'scope':'Six selected source report byte assertions with predeclared source-exposed author expectations. No financial evaluator/fullProgram/compiler/native/ledger or parent EARS acceptance. Originalpatch trailing blankline removed by rustfmt in test source only, allfrozenfixtures unchanged.'})
 print(json.dumps({'selectedSixCaseSuiteCompleted':len(results)==1,'ownedRootRemoved':not root.exists()}),flush=True)
