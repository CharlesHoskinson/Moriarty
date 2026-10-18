from pathlib import Path
import datetime, hashlib, json, shutil, subprocess, tempfile, time

R=Path('/home/charl/moriarty-c2-opus55-restored-20261017')
J=Path('/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018')
C=R/'core-consumer-roundtrips-check-cycle-20261018'
C.mkdir()
HEAD='bcdfd4b27a62e8e618153ef1961d60f889dc4d63'
TC=Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin')
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def now(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def write(name,x): (C/name).write_text(json.dumps(x,indent=2)+'\n')
def git(*args): return subprocess.check_output(['git',*args],cwd=J).decode().strip()
assert git('rev-parse','HEAD')==HEAD and not git('status','--porcelain=v1')
tree=git('rev-parse','HEAD^{tree}')
frozen=[]
for folder,expected in [('utxo-pattern-core-independent-oracle-20261018','9f4ba8ddac07c4823eb2199e0ce2866e54b3c424221ce2cea3172a9660063d00'),('program-plan-roundtrip-independent-oracle-20261018','904f438ad11cdcee5698b62973cbdae3d977c53275f86427cdecc42b366589f4')]:
    root=R/folder; assert sha(root/'FREEZE.sha256')==expected
    count=0
    for line in (root/'FREEZE.sha256').read_text().splitlines():
        digest,path=line.split(maxsplit=1);path=path.lstrip('*')
        assert not Path(path).is_absolute() and '..' not in Path(path).parts
        assert sha(root/path)==digest,path
        count+=1
    frozen.append({'path':str(root),'freezeSha256':expected,'membersVerified':count})
paths=subprocess.check_output(['git','ls-tree','-r','--name-only',HEAD,'crates','Cargo.toml','Cargo.lock','rust-toolchain.toml','RUST_GUIDELINES.md'],cwd=J).decode().splitlines()
pins=[]
for path in paths:
    raw=subprocess.check_output(['git','show',HEAD+':'+path],cwd=J)
    assert (J/path).read_bytes()==raw,path
    pins.append({'path':path,'sha256':hashlib.sha256(raw).hexdigest()})
write('SOURCE-PINS.json',{'head':HEAD,'tree':tree,'files':pins})
root=Path(tempfile.mkdtemp(prefix='doctorwatson-core-roundtrips-'))
for name in ['cwd','tmp','cargo-home']: (root/name).mkdir()
for name in ['registry','git']: (root/'cargo-home'/name).symlink_to(Path('/home/charl/.cargo')/name,target_is_directory=True)
env={'PATH':str(TC)+':/usr/bin:/bin','RUSTC':str(TC/'rustc'),'RUSTDOC':str(TC/'rustdoc'),'CARGO_HOME':str(root/'cargo-home'),'CARGO_TARGET_DIR':str(root/'target'),'CARGO_NET_OFFLINE':'true','CARGO_INCREMENTAL':'0','CARGO_CACHE_AUTO_CLEAN_FREQUENCY':'never','GIT_CONFIG_NOSYSTEM':'1','GIT_CONFIG_GLOBAL':'/dev/null','GIT_CONFIG_SYSTEM':'/dev/null','LANG':'C.UTF-8','TMPDIR':str(root/'tmp'),'MORI_UTXO_PATTERN_EXECUTION_OUTPUT':str(C/'ACTUAL-UTXO-EXECUTION.json')}
configs=[]
for directory in [J,*J.parents,root/'cwd',*(root/'cwd').parents]:
    for filename in ['config','config.toml']:
        p=directory/'.cargo'/filename
        if str(p) not in configs:
            assert not p.exists(),str(p)
            configs.append(str(p))
argv=['/usr/bin/timeout','--signal=TERM','--kill-after=5s','90s',str(TC/'cargo'),'test','--manifest-path',str(J/'Cargo.toml'),'--frozen','-j1','-p','mori-core','--no-default-features','--test','program_plan_roundtrip','--test','utxo_pattern_exchange','--test','financial_program_wire_contract','--','--test-threads=1']
write('CHECK-PLAN.json',{'recordedAt':now(),'head':HEAD,'tree':tree,'base':'b2786e8123e8d9327265f0fbb373444a7cb30f16','argv':argv,'environment':env,'ownedTemporaryRoot':str(root),'absentAncestorConfigs':configs,'toolPins':[{'path':str(p),'sha256':sha(p)} for p in [TC/'cargo',TC/'rustc',TC/'rustdoc',Path('/usr/bin/timeout')]],'independentFreezesVerifiedBeforeRun':frozen,'scope':'Three actual ordinary Core integration targets, four tests; mandatory original compiler/SDK/native/proof/target/financial-family acceptance and consumed SDK allocations unchanged. No native or midnight-compat-tests feature selected.','expectation':'Program/Plan/Continuation whole values; two financial full Program/v1 wires; UTXO complete11prefixes/6effects/preserved duties. No prior passing target repeated.'})
result=None
try:
    start=time.monotonic()
    write('START.json',{'startedAt':now(),'head':HEAD,'tree':tree,'argv':argv})
    with (C/'STDOUT.log').open('wb') as out,(C/'STDERR.log').open('wb') as err:
        child=subprocess.Popen(argv,cwd=root/'cwd',env=env,stdin=subprocess.DEVNULL,stdout=out,stderr=err,start_new_session=True)
        write('CHILD.json',{'pid':child.pid,'session':child.pid})
        code=child.wait()
    remaining=[int(p) for p,s in (line.split() for line in subprocess.check_output(['ps','-eo','pid=,sid=']).decode().splitlines()) if int(s)==child.pid]
    result={'finishedAt':now(),'exitCode':code,'wallSeconds':time.monotonic()-start,'stdoutSha256':sha(C/'STDOUT.log'),'stderrSha256':sha(C/'STDERR.log'),'ownedSessionRemaining':remaining,'head':HEAD,'tree':tree,'sourcePinsUnchanged':all(sha(J/p['path'])==p['sha256'] for p in pins),'workingTreeClean':not git('status','--porcelain=v1')}
    write('RESULT.json',result);print(json.dumps(result),flush=True)
    assert code==0 and not remaining and result['sourcePinsUnchanged'] and result['workingTreeClean']
    text=(C/'STDOUT.log').read_text()
    assert text.count('test result: ok.')==3 and '2 passed; 0 failed' in text and text.count('1 passed; 0 failed')==2
    actual=json.loads((C/'ACTUAL-UTXO-EXECUTION.json').read_text());oracle=json.loads((R/'utxo-pattern-core-independent-oracle-20261018/COMPLETE-PREFIXES.expected.json').read_text())
    for key in ['semantics','action','pre','post','effects']: assert actual[key]==oracle[key],key
    effects=[]
    for step,prefix in zip(actual['steps'],oracle['prefixes'],strict=True):
        assert step['snapshot']==prefix['snapshot'] and step['effects']==prefix['stepEffects'] and step['index']==prefix['index']
        effects.extend(step['effects']);assert effects==prefix['cumulativeEffects']
    write('ORACLE-COMPARISON.json',{'actualExecutionSha256':sha(C/'ACTUAL-UTXO-EXECUTION.json'),'oracleSha256':sha(R/'utxo-pattern-core-independent-oracle-20261018/COMPLETE-PREFIXES.expected.json'),'completePrefixCount':len(actual['steps']),'orderedEffectCount':len(effects),'completePrePostAndAllPrefixSnapshotsAndDeltaCumulativeEffectsEqual':True,'limits':'One local finite exchange; Statement/ExprId/Step.value only separately authored expectations. No all20family or target/compiler/native qualification.'})
finally:
    shutil.rmtree(root)
    write('CLEANUP.json',{'recordedAt':now(),'ownedRoot':str(root),'existsAfter':root.exists(),'resultRecorded':result is not None})
