from pathlib import Path
import datetime, hashlib, json, shutil, subprocess, tempfile, time

R=Path('/home/charl/moriarty-c2-opus55-restored-20261017')
J=Path('/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018')
C=R/'payout-reserve-check-cycle-20261018'
C.mkdir()
HEAD='de1eaf81eec57c7cec48c2ddae57362ff2c7d498'
TC=Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin')
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def now(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def write(name,x): (C/name).write_text(json.dumps(x,indent=2)+'\n')
def git(*args): return subprocess.check_output(['git',*args],cwd=J).decode().strip()
assert git('rev-parse','HEAD')==HEAD and not git('status','--porcelain=v1')
tree=git('rev-parse','HEAD^{tree}')
frozen=[]
payout=R/'concurrency32-wave-20261018/publication_fixture_oracle'
assert sha(payout/'FREEZE.json')=='b78c25666d50fb2f64b25fd809306df0e6f414a42284d8069764ee064a857a7e'
assert sha(payout/'MANIFEST.json')=='329c67fa15b238e09dfd9ef705424975d5a070bd2b975fb38a4ff9210aaf483f'
pm=json.loads((payout/'MANIFEST.json').read_text());assert len(pm['files'])==26
for item in pm['files']:
    assert sha(payout/item['path'])==item['sha256']
    assert (J/'crates/fixtures/three-way-payout-contract'/item['path']).read_bytes()==(payout/item['path']).read_bytes()
frozen.append({'path':str(payout),'freezeSha256':sha(payout/'FREEZE.json'),'membersVerified':26,'conditionalPremises':json.loads((payout/'FREEZE.json').read_text())['conditional_representation']})
reserve=R/'concurrency32-wave-20261018/two_program_financial_oracles'
assert sha(reserve/'FREEZE.sha256')=='1b606141dff7ca162bcd132d55d62014187de4f936cfc6528c23c8d22591aefb'
count=0
for line in (reserve/'FREEZE.sha256').read_text().splitlines():
    digest,path=line.split(maxsplit=1);path=path.lstrip('*');assert not Path(path).is_absolute() and '..' not in Path(path).parts;assert sha(reserve/path)==digest;count+=1
assert count==25
frozen.append({'path':str(reserve),'freezeSha256':sha(reserve/'FREEZE.sha256'),'membersVerified':count})
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
env={'PATH':str(TC)+':/usr/bin:/bin','RUSTC':str(TC/'rustc'),'RUSTDOC':str(TC/'rustdoc'),'CARGO_HOME':str(root/'cargo-home'),'CARGO_TARGET_DIR':str(root/'target'),'CARGO_NET_OFFLINE':'true','CARGO_INCREMENTAL':'0','CARGO_CACHE_AUTO_CLEAN_FREQUENCY':'never','GIT_CONFIG_NOSYSTEM':'1','GIT_CONFIG_GLOBAL':'/dev/null','GIT_CONFIG_SYSTEM':'/dev/null','LANG':'C.UTF-8','TMPDIR':str(root/'tmp'),'MORI_PAYOUT_OBSERVATIONS':str(C/'payout-observations'),'MORI_CORE_REFUND_OBSERVATIONS':str(C/'reserve-observations')}
configs=[]
for directory in [J,*J.parents,root/'cwd',*(root/'cwd').parents]:
    for filename in ['config','config.toml']:
        p=directory/'.cargo'/filename
        if str(p) not in configs:
            assert not p.exists(),str(p)
            configs.append(str(p))
argv=['/usr/bin/timeout','--signal=TERM','--kill-after=5s','90s',str(TC/'cargo'),'test','--manifest-path',str(J/'Cargo.toml'),'--frozen','-j1','-p','mori','--no-default-features','--test','core_three_way_payout','--test','core_reserve_spend_refund','--','--test-threads=1']
write('CHECK-PLAN.json',{'recordedAt':now(),'head':HEAD,'tree':tree,'base':'049395532c43b17c54bdbc5a529535b521dc56df','argv':argv,'environment':env,'ownedTemporaryRoot':str(root),'absentAncestorConfigs':configs,'toolPins':[{'path':str(p),'sha256':sha(p)} for p in [TC/'cargo',TC/'rustc',TC/'rustdoc',Path('/usr/bin/timeout')]],'independentFreezesVerifiedBeforeRun':frozen,'scope':'Two actual ordinary public CLI/Core financial targets, five tests; mandatory original compiler/SDK/native/proof/target/financial-family acceptance and consumed SDK allocations unchanged. No native or midnight-compat-tests feature selected.','expectation':'Three-recipient payout full Program/Execution/cost/CLI candidates and4financialprefixes; reserve partialspend/refund20financialprefixes/17effects through library+CLI. Wholepayoutarena/byteformatpremisesconditional; no allowance primitive enforced. No old passing tests rerun.'})
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
    assert text.count('test result: ok.')==2 and '3 passed; 0 failed' in text and '2 passed; 0 failed' in text
    observed=json.loads((C/'payout-observations/ordinary-execution.json').read_text())
    assert observed==json.loads((payout/'three-way-payout.execution.expected.json').read_text())
    write('CAPTURE-INVENTORY.json',{'files':[{'path':str(p.relative_to(C)),'bytes':p.stat().st_size,'sha256':sha(p)} for directory in [C/'payout-observations',C/'reserve-observations'] for p in sorted(directory.rglob('*')) if p.is_file()],'scope':'Complete actual captures retained, separate from independent financial and conditional wire/cost premises.'})

finally:
    shutil.rmtree(root)
    write('CLEANUP.json',{'recordedAt':now(),'ownedRoot':str(root),'existsAfter':root.exists(),'resultRecorded':result is not None})
