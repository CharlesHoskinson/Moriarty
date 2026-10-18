from pathlib import Path
import datetime, hashlib, json, shutil, subprocess, tempfile, time

R=Path('/home/charl/moriarty-c2-opus55-restored-20261017')
K=Path('/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018')
C=R/'shared-check-capability-integration-check-cycle-20261018'
C.mkdir()
BASELINE=R/'shared-check-production-check-cycle'
HEAD='64c6dedff1d99b33c23832330d7911a5fad9ed59'
TC=Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin')
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def now(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def write(name,x):
    p=C/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(x,indent=2)+'\n')
def git(*args): return subprocess.check_output(['git',*args],cwd=K).decode().strip()
assert git('rev-parse','HEAD')==HEAD and not git('status','--porcelain=v1')
tree=git('rev-parse','HEAD^{tree}')
cases=json.loads((BASELINE/'INPUT-CASES-FROZEN.json').read_text());assert len(cases)==8
assert sha(BASELINE/'INPUT-CASES-FROZEN.json')=='212706c623973d936e1ea93649088f42ae3c7cddf853b5d35800aed859534478'
shutil.copyfile(BASELINE/'INPUT-CASES-FROZEN.json',C/'INPUT-CASES-FROZEN.json')
paths=subprocess.check_output(['git','ls-tree','-r','--name-only',HEAD,'crates','Cargo.toml','Cargo.lock','rust-toolchain.toml'],cwd=K).decode().splitlines();pins=[]
for path in paths:
    raw=subprocess.check_output(['git','show',HEAD+':'+path],cwd=K);assert (K/path).read_bytes()==raw,path
    pins.append({'path':path,'sha256':hashlib.sha256(raw).hexdigest()})
write('SOURCE-PINS.json',{'head':HEAD,'tree':tree,'files':pins})
root=Path(tempfile.mkdtemp(prefix='doctorwatson-shared-main-integration-'))
for name in ['cwd','tmp','cargo-home']: (root/name).mkdir()
for name in ['registry','git']: (root/'cargo-home'/name).symlink_to(Path('/home/charl/.cargo')/name,target_is_directory=True)
env={'PATH':str(TC)+':/usr/bin:/bin','RUSTC':str(TC/'rustc'),'RUSTDOC':str(TC/'rustdoc'),'CARGO_HOME':str(root/'cargo-home'),'CARGO_TARGET_DIR':str(root/'target'),'CARGO_NET_OFFLINE':'true','CARGO_INCREMENTAL':'0','CARGO_CACHE_AUTO_CLEAN_FREQUENCY':'never','GIT_CONFIG_NOSYSTEM':'1','GIT_CONFIG_GLOBAL':'/dev/null','GIT_CONFIG_SYSTEM':'/dev/null','LANG':'C.UTF-8','TMPDIR':str(root/'tmp')}
configs=[]
for directory in [K,*K.parents,root/'cwd',*(root/'cwd').parents]:
    for filename in ['config','config.toml']:
        p=directory/'.cargo'/filename
        if str(p) not in configs:assert not p.exists(),str(p);configs.append(str(p))
write('CHECK-PLAN.json',{'recordedAt':now(),'head':HEAD,'tree':tree,'parents':git('show','-s','--format=%P','HEAD').split(),'mainBase':'b2786e8123e8d9327265f0fbb373444a7cb30f16','environment':env,'ownedTemporaryRoot':str(root),'absentAncestorConfigs':configs,'toolPins':[{'path':str(p),'sha256':sha(p)} for p in [TC/'cargo',TC/'rustc',TC/'rustdoc',Path('/usr/bin/timeout')]],'sourceInputsFreezeSha256':sha(C/'INPUT-CASES-FROZEN.json'),'scope':'Actual current-main integration of production shared source checker and approved capabilities command. Ten selected public CLI tests plus eight complete baseline stdout/exit comparisons. No SDK/native/MCP/proof/wallet/ledger execution or consumed allocation reset.'})
def run(name,cmd,bound,cwd):
    d=C/name;d.mkdir(parents=True);argv=['/usr/bin/timeout','--signal=TERM','--kill-after=5s',str(bound)+'s',*cmd];write(name+'/START.json',{'startedAt':now(),'argv':argv,'cwd':str(cwd),'environment':env,'head':HEAD,'tree':tree})
    start=time.monotonic()
    with (d/'STDOUT.log').open('wb') as out,(d/'STDERR.log').open('wb') as err:
        child=subprocess.Popen(argv,cwd=cwd,env=env,stdin=subprocess.DEVNULL,stdout=out,stderr=err,start_new_session=True);write(name+'/CHILD.json',{'pid':child.pid,'session':child.pid});code=child.wait()
    remaining=[int(p) for p,s in (line.split() for line in subprocess.check_output(['ps','-eo','pid=,sid=']).decode().splitlines()) if int(s)==child.pid]
    result={'finishedAt':now(),'exitCode':code,'wallSeconds':time.monotonic()-start,'stdoutSha256':sha(d/'STDOUT.log'),'stderrSha256':sha(d/'STDERR.log'),'ownedSessionRemaining':remaining};write(name+'/RESULT.json',result);assert not remaining;return result,d
tests=None;observed=[]
try:
    tests,d=run('public-tests',[str(TC/'cargo'),'test','--manifest-path',str(K/'Cargo.toml'),'--frozen','-j1','-p','mori','--no-default-features','--test','check_source_dispatch','--test','capabilities','--','--test-threads=1'],90,root/'cwd');print(json.dumps({'tests':tests}),flush=True);assert tests['exitCode']==0
    text=(d/'STDOUT.log').read_text();assert text.count('test result: ok.')==2 and '2 passed; 0 failed' in text and '8 passed; 0 failed' in text
    for case in cases:
        cwd=root/'cwd'/case['name'];cwd.mkdir();(cwd/case['file']).write_text(case['source'])
        result,d=run('direct-cases/'+case['name'],[str(root/'target/debug/mori'),'--color','never','check',case['file'],'--json'],10,cwd)
        baseline=BASELINE/'baseline-cases'/case['name'];expected=json.loads((baseline/'RESULT.json').read_text())['exitCode'];same=(d/'STDOUT.log').read_bytes()==(baseline/'STDOUT.log').read_bytes()
        row={'name':case['name'],**result,'sameCompleteStdoutBytesAsActualBaseline':same,'sameExitAsActualBaseline':result['exitCode']==expected,'stderrEmpty':not (d/'STDERR.log').read_bytes(),'baselineStdoutSha256':sha(baseline/'STDOUT.log'),'baselineResultSha256':sha(baseline/'RESULT.json')};observed.append(row);assert same and row['sameExitAsActualBaseline'] and row['stderrEmpty']
    assert git('rev-parse','HEAD')==HEAD and not git('status','--porcelain=v1') and all(sha(K/p['path'])==p['sha256'] for p in pins)
    print(json.dumps({'completedDirectComparisons':len(observed),'allCompleteBytesAndExitsEqual':True}),flush=True)
finally:
    shutil.rmtree(root)
    write('FINAL-RESULT.json',{'finishedAt':now(),'head':HEAD,'tree':tree,'tests':tests,'directObservations':observed,'allEightCompleted':len(observed)==8,'ownedTemporaryRoot':str(root),'ownedRootExistsAfter':root.exists(),'scope':'Ten selected normal CLI tests and8actualbaseline compatibility comparisons on current-main combined tree; no independent semantic/MCP/native/proof/ledger/target qualification.'})
    write('CLEANUP.json',{'recordedAt':now(),'ownedRoot':str(root),'existsAfter':root.exists()})
