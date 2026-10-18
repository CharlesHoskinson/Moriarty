from pathlib import Path
import datetime,hashlib,json,os,shutil,subprocess,tempfile,time
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017')
K=Path('/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018')
P=R/'public-cli-profile-guidance-main79-20261018'
C=R/'cli-finance-main238-combined-check-cycle-20261018';C.mkdir()
HEAD='f91f2c7f1b3bf2229d1243fc6a1f9ee37004a209'
TC=Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def write(name,x):
 p=C/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(x,indent=2)+'\n')
def git(*args):return subprocess.check_output(['git',*args],cwd=K).decode().strip()
assert git('rev-parse','HEAD')==HEAD and not git('status','--porcelain=v1')
tree=git('rev-parse','HEAD^{tree}')
assert tree=='ed8323c00f31deaace461bf63db0365e8c4e0aac'
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
write('CHECK-PLAN.json',{'recordedAt':now(),'head':HEAD,'tree':tree,'base':'71c22f8a477231be869786eb42f04e3b50d1b3a7','sourceManifestSha256':sha(P/'MANIFEST.json'),'environment':env,'ownedRoot':str(root),'absentAncestorCargoConfigs':configs,'toolPins':[{'path':str(p),'sha256':sha(p)} for p in [TC/'cargo',TC/'rustc',TC/'rustdoc',Path('/usr/bin/timeout'),Path('/usr/bin/bash')]],'scope':'Exact integrated CLI/schema and public header regression plus six public source-check cases and five syntax commands. Existing per-command90s ceilings; no evaluator/SDK/native/proof/key/wallet/ledger action.'})
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
 result,d=run('clap-help-test',[str(TC/'cargo'),'test','--manifest-path',str(K/'Cargo.toml'),'--frozen','-j1','-p','mori','--no-default-features','--bin','mori','cli::tests::help_distinguishes_core_consumers_from_source6_proposals','--','--exact','--test-threads=1'],90)
 results.append({'name':'clap-help-test',**result});print(json.dumps(results[-1]),flush=True)
 assert result['exitCode']==0
 text=(d/'STDOUT.log').read_text();assert '1 passed; 0 failed' in text and 'help_distinguishes_core_consumers_from_source6_proposals ... ok' in text
 result,d=run('financial-source-check-tests',[str(TC/'cargo'),'test','--manifest-path',str(K/'Cargo.toml'),'--frozen','-j1','-p','mori','--no-default-features','--test','check_source_dispatch','financial_','--','--test-threads=1'],90)
 results.append({'name':'financial-source-check-tests',**result});print(json.dumps(results[-1]),flush=True)
 assert result['exitCode']==0
 text=(d/'STDOUT.log').read_text();assert '6 passed; 0 failed' in text and '8 filtered out' in text and 'financial_core2_rebate ... ok' in text
 result,d=run('language-header-test',[str(TC/'cargo'),'test','--manifest-path',str(K/'Cargo.toml'),'--frozen','-j1','-p','mori-core','--no-default-features','--test','language','language_header_diagnostics_name_both_supported_core_profiles','--','--exact','--test-threads=1'],90)
 results.append({'name':'language-header-test',**result});print(json.dumps(results[-1]),flush=True)
 assert result['exitCode']==0
 text=(d/'STDOUT.log').read_text();assert '1 passed; 0 failed' in text and 'language_header_diagnostics_name_both_supported_core_profiles ... ok' in text
 for block in manifest['rootShellSyntaxChecks']:
  file=P/block['path'];assert sha(file)==block['sha256']
  result,d=run('shell-'+str(block['block']),['/usr/bin/bash','-n',str(file)],10);results.append({'name':'shell-'+str(block['block']),**result});assert result['exitCode']==0 and not(d/'STDOUT.log').read_bytes()and not(d/'STDERR.log').read_bytes()
 assert git('rev-parse','HEAD')==HEAD and not git('status','--porcelain=v1')and all(sha(K/p['path'])==p['sha256']for p in pins)
finally:
 shutil.rmtree(root)
 write('FINAL-RESULT.json',{'finishedAt':now(),'head':HEAD,'tree':tree,'results':results,'allEightCommandsCompleted':len(results)==8,'ownedRoot':str(root),'existsAfter':root.exists(),'scope':'Eight selected test cases, five shell syntax commands. Financial expectations remain exposed-author fixtures; twelve CLI byte assertions execute inside selected six tests, no separate twelve-stream captures. Full language/compiler/proof/native/ledger acceptance remains open.'})
 print(json.dumps({'allEightCommandsCompleted':len(results)==8,'ownedRootRemoved':not root.exists()}),flush=True)
