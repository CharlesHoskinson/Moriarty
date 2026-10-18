from pathlib import Path
import subprocess,datetime,time,json,hashlib,tempfile,shutil,os
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017'); K=Path('/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018'); C=R/'core2-quint-shared-projection-check-cycle/actual-check-a1'; O=R/'reserve-quint-independent-oracle-20261018'; T=R/'quint-current-toolchain-custody-20261018'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def write(p,x):p.write_text(json.dumps(x,indent=2)+'\n')
def git(*a):return subprocess.check_output(['git',*a],cwd=K).decode().strip()
C.mkdir();head=git('rev-parse','HEAD');tree=git('rev-parse','HEAD^{tree}');assert head=='f15214ab7a885a4dc3194002cbacbb56841fdcbf' and not git('status','--porcelain=v1')
assert sha(O/'FREEZE.sha256')=='7dc1b5c503a7cd2ab8e298fa638d42d7afd365df5e6253ce280e99af2178e7c3'
lines=(O/'FREEZE.sha256').read_text().splitlines();assert len(lines)==35
for line in lines:
 h,name=line.split(maxsplit=1);assert sha(O/name.lstrip('*'))==h,name
expected=json.loads((O/'reserve-common-projection.expected.json').read_text());fixture=K/'crates/mori-core/tests/fixtures/continuations/reserve-quint-projection.json';assert expected==json.loads(fixture.read_text())
paths=subprocess.check_output(['git','ls-tree','-r','--name-only',head,'crates/mori-core','crates/mori-lexer','examples/core/continuations','models/core-stage-correspondence','docs/audits/2026-10-15/core-continuations/quint-lifecycle','Cargo.toml','Cargo.lock','rust-toolchain.toml'],cwd=K).decode().splitlines();pins={}
for path in paths:
 data=subprocess.check_output(['git','show',head+':'+path],cwd=K);assert (K/path).read_bytes()==data,path;pins[path]={'path':path,'sha256':hashlib.sha256(data).hexdigest(),'bytes':len(data),'gitBlob':git('rev-parse',head+':'+path)}
write(C/'SOURCE-PINS.json',{'commit':head,'tree':tree,'files':list(pins.values())})
root=Path(tempfile.mkdtemp(prefix='doctorwatson-reserve-quint-'));[ (root/n).mkdir() for n in ['cwd','tmp','cargo-home','quint-home'] ]
for name in ['registry','git']:(root/'cargo-home'/name).symlink_to(Path('/home/charl/.cargo')/name,target_is_directory=True)
toolchain=Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin');node=Path('/home/charl/.foreman/tools/fnm/node-versions/v24.21.0/installation/bin/node');cli=Path('/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cli.js');tools=[toolchain/'cargo',toolchain/'rustc',toolchain/'rustdoc',Path('/usr/bin/timeout'),node,cli];toolpins=[{'path':str(p),'resolved':str(p.resolve()),'sha256':sha(p.resolve())} for p in tools]
configs=[]
for directory in [K,*K.parents,root/'cwd',*(root/'cwd').parents]:
 for filename in ['config','config.toml']:
  p=directory/'.cargo'/filename
  if str(p) not in configs:assert not p.exists(),str(p);configs.append(str(p))
renv={'PATH':str(toolchain)+':/usr/bin:/bin','RUSTC':str(toolchain/'rustc'),'RUSTDOC':str(toolchain/'rustdoc'),'CARGO_HOME':str(root/'cargo-home'),'CARGO_TARGET_DIR':str(root/'target'),'CARGO_NET_OFFLINE':'true','CARGO_INCREMENTAL':'0','CARGO_CACHE_AUTO_CLEAN_FREQUENCY':'never','GIT_CONFIG_NOSYSTEM':'1','GIT_CONFIG_GLOBAL':'/dev/null','GIT_CONFIG_SYSTEM':'/dev/null','LANG':'C.UTF-8','TMPDIR':str(root/'tmp')}
qenv={'PATH':'/usr/bin:/bin','LANG':'C.UTF-8','LC_ALL':'C.UTF-8','TZ':'UTC','NO_COLOR':'1','QUINT_HOME':str(root/'quint-home')}
model=str(K/'models/core-stage-correspondence/reserve_shared_projection_test.qnt');capture=str(C/'reserve_{test}_{seq}.itf.json')
base=[str(toolchain/'cargo'),'test','--manifest-path',str(K/'Cargo.toml'),'--frozen','-j1','-p','mori-core','--no-default-features']
steps=[('rust-shared-projection',90,renv,base+['--test','reserve_quint_projection','--','--exact','reserve_existing_trace_matches_complete_shared_quint_projection','--test-threads=1','--nocapture']),('rust-continuation-helper',90,renv,base+['--test','continuations','--','--test-threads=1']),('quint-typecheck',20,qenv,[str(node),'--max-old-space-size=256',str(cli),'typecheck',model]),('quint-deterministic-trace',20,qenv,[str(node),'--max-old-space-size=256',str(cli),'test',model,'--main=reserve_shared_projection_test','--backend=typescript','--match=^reserveSharedProjectionTraceTest$','--max-samples=1','--seed=20261018','--verbosity=2','--out-itf='+capture])]
write(C/'CHECK-PLAN.json',{'recordedAt':now(),'commit':head,'tree':tree,'sourcePins':len(pins),'independentOracleFreezeSha256':sha(O/'FREEZE.sha256'),'independentOracleMembersVerified':35,'oracleEqualsAuthoredFixture':True,'ownedTemporaryRoot':str(root),'toolPins':toolpins,'absentAncestorConfigs':configs,'steps':[{'name':n,'wallSeconds':b,'killGraceSeconds':5,'argv':a,'environment':e} for n,b,e,a in steps],'authority':'Inspected ordinary Core tests and explicit TypeScript finite Quint test, no native SDK/VM/proof/key/wallet/ledger execution. No reset of previous consumed limits.','scope':'One existing reserve trajectory, five captured lifecycle states and independent finance/work. Not exhaustive model proof or target correspondence.','attemptsPerStep':1,'nodeHeapLimitMiB':256,'heapNotRSS':True})
results=[]
try:
 for name,bound,env,cmd in steps:
  assert git('rev-parse','HEAD')==head and not git('status','--porcelain=v1')
  for p,row in pins.items():assert sha(K/p)==row['sha256'],p
  d=C/name;d.mkdir();argv=['/usr/bin/timeout','--signal=TERM','--kill-after=5s',str(bound)+'s',*cmd];write(d/'START.json',{'startedAt':now(),'commit':head,'tree':tree,'argv':argv,'cwd':str(root/'cwd'),'environment':env,'sourcePinsVerified':len(pins),'attempt':1})
  start=time.monotonic()
  with (d/'STDOUT.log').open('wb') as out,(d/'STDERR.log').open('wb') as err:
   child=subprocess.Popen(argv,cwd=root/'cwd',env=env,stdout=out,stderr=err,start_new_session=True);sid=child.pid;write(d/'CHILD.json',{'pid':sid,'session':sid});code=child.wait()
  remaining=[int(p) for p,s in (line.split() for line in subprocess.check_output(['ps','-eo','pid=,sid=']).decode().splitlines()) if int(s)==sid]
  write(d/'PROCESS-CLEANUP.json',{'ownedSession':sid,'remainingSessionMembers':remaining,'scope':'OwnedSID census only.'})
  result={'finishedAt':now(),'name':name,'exitCode':code,'wallSeconds':time.monotonic()-start,'stdoutSha256':sha(d/'STDOUT.log'),'stderrSha256':sha(d/'STDERR.log'),'commitAfter':git('rev-parse','HEAD'),'treeAfter':git('rev-parse','HEAD^{tree}'),'statusAfter':git('status','--porcelain=v1')};write(d/'RESULT.json',result);results.append(result);print(json.dumps(result),flush=True)
  assert not remaining and code==0 and result['commitAfter']==head and result['treeAfter']==tree and not result['statusAfter'],result
  out=(d/'STDOUT.log').read_text()
  if name=='rust-shared-projection':assert '1 passed; 0 failed' in out and out.count('RESERVE_QUINT_PROJECTION ')==1
  if name=='rust-continuation-helper':assert '5 passed; 0 failed' in out
  if name=='quint-deterministic-trace':
   assert '1 passing' in out,out
   captures=list(C.glob('*.itf.json'));assert len(captures)==1,captures
   data=json.loads(captures[0].read_text());assert len(data['states'])==5,data.keys();write(C/'CAPTURE-SUMMARY.json',{'path':str(captures[0]),'sha256':sha(captures[0]),'vars':data['vars'],'states':len(data['states']),'semanticDecoding':'PENDING independent source-grounded decoder'})
finally:
 shutil.rmtree(root);write(C/'CLEANUP.json',{'recordedAt':now(),'ownedTemporaryRoot':str(root),'existsAfter':root.exists(),'scope':'Only owned temp root deleted, cache symlinks not traversed.'});write(C/'RESULT.json',{'recordedAt':now(),'commit':head,'tree':tree,'steps':results,'allStepsCompleted':len(results)==4,'allExitZero':len(results)==4 and all(r['exitCode']==0 for r in results),'temporaryRootExistsAfter':root.exists(),'semanticCapturedStreamComparison':'PENDING; exit0 and literal assertions alone are not full stream equality.'})
