from pathlib import Path
import datetime, hashlib, json, shutil, subprocess, tempfile, time

R=Path('/home/charl/moriarty-c2-opus55-restored-20261017')
J=Path('/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018')
C=R/'escrow-invoice-check-cycle-20261018'
C.mkdir()
HEAD='68a46a3387faa051a26324e58e78e0a830ecbaed'
TC=Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin')
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def now(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def write(name,x): (C/name).write_text(json.dumps(x,indent=2)+'\n')
def git(*args): return subprocess.check_output(['git',*args],cwd=J).decode().strip()
assert git('rev-parse','HEAD')==HEAD and not git('status','--porcelain=v1')
tree=git('rev-parse','HEAD^{tree}')
frozen=[]
for folder,expected in [('concurrency32-wave-20261018/utxo_devex_independent_oracle_sol','367b32a2089c210edd4830c4f2274d73ccf2c97d3132323f22ebc723efe54959'),('concurrency32-wave-20261018/financial_report_contract_sol','b8f6499a8b2d3caa856b0489f0754fbb7ed653731a0971ec0da5cdeac61acac6'),('program-plan-roundtrip-independent-oracle-20261018','904f438ad11cdcee5698b62973cbdae3d977c53275f86427cdecc42b366589f4')]:
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
env={'PATH':str(TC)+':/usr/bin:/bin','RUSTC':str(TC/'rustc'),'RUSTDOC':str(TC/'rustdoc'),'CARGO_HOME':str(root/'cargo-home'),'CARGO_TARGET_DIR':str(root/'target'),'CARGO_NET_OFFLINE':'true','CARGO_INCREMENTAL':'0','CARGO_CACHE_AUTO_CLEAN_FREQUENCY':'never','GIT_CONFIG_NOSYSTEM':'1','GIT_CONFIG_GLOBAL':'/dev/null','GIT_CONFIG_SYSTEM':'/dev/null','LANG':'C.UTF-8','TMPDIR':str(root/'tmp'),'MORI_UTXO_PATTERN_EXECUTION_OUTPUT':str(C/'ACTUAL-ESCROW-EXECUTION.json'),'MORI_INVOICE_RECEIPT_CAPTURE_DIR':str(C/'invoice-captures')}
(C/'invoice-captures').mkdir()
configs=[]
for directory in [J,*J.parents,root/'cwd',*(root/'cwd').parents]:
    for filename in ['config','config.toml']:
        p=directory/'.cargo'/filename
        if str(p) not in configs:
            assert not p.exists(),str(p)
            configs.append(str(p))
argv=['/usr/bin/timeout','--signal=TERM','--kill-after=5s','90s',str(TC/'cargo'),'test','--manifest-path',str(J/'Cargo.toml'),'--frozen','-j1','-p','mori-core','--no-default-features','--test','program_plan_roundtrip','--test','utxo_escrow_partial_release','--test','invoice_receipt_resume','--','--test-threads=1']
write('CHECK-PLAN.json',{'recordedAt':now(),'head':HEAD,'tree':tree,'base':'049395532c43b17c54bdbc5a529535b521dc56df','argv':argv,'environment':env,'ownedTemporaryRoot':str(root),'absentAncestorConfigs':configs,'toolPins':[{'path':str(p),'sha256':sha(p)} for p in [TC/'cargo',TC/'rustc',TC/'rustdoc',Path('/usr/bin/timeout')]],'independentFreezesVerifiedBeforeRun':frozen,'scope':'Three actual ordinary Core integration targets, three changed tests; mandatory original compiler/SDK/native/proof/target/financial-family acceptance and consumed SDK allocations unchanged. No native or midnight-compat-tests feature selected.','expectation':'Complete Waiting prefix and continuation; actual successful receipt resume with full semantic suffix and cross-segment prefixes; partial escrow11prefixes/6effects/preserved duties. Existing waiting test gains complete assertions; no unrelated prior passing target repeated.'})
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
    assert text.count('test result: ok.')==3 and text.count('1 passed; 0 failed')==3
    actual=json.loads((C/'ACTUAL-ESCROW-EXECUTION.json').read_text());oracle=json.loads((R/'concurrency32-wave-20261018/utxo_devex_independent_oracle_sol/COMPLETE-PREFIXES.expected.json').read_text())
    for key in ['semantics','action','pre','post','effects']: assert actual[key]==oracle[key],key
    effects=[]
    for step,prefix in zip(actual['steps'],oracle['prefixes'],strict=True):
        assert step['snapshot']==prefix['snapshot'] and step['effects']==prefix['stepEffects'] and step['index']==prefix['index']
        effects.extend(step['effects']);assert effects==prefix['cumulativeEffects']
    write('ORACLE-COMPARISON.json',{'actualExecutionSha256':sha(C/'ACTUAL-ESCROW-EXECUTION.json'),'oracleSha256':sha(R/'concurrency32-wave-20261018/utxo_devex_independent_oracle_sol/COMPLETE-PREFIXES.expected.json'),'completePrefixCount':len(actual['steps']),'orderedEffectCount':len(effects),'completePrePostAndAllPrefixSnapshotsAndDeltaCumulativeEffectsEqual':True,'limits':'One local partial escrow; Statement/ExprId/Step.value outside independent oracle. Invoice suffix separately captured/compared by actual test. No all20family or target/compiler/native qualification.'})
finally:
    shutil.rmtree(root)
    write('CLEANUP.json',{'recordedAt':now(),'ownedRoot':str(root),'existsAfter':root.exists(),'resultRecorded':result is not None})
