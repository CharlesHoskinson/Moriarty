from pathlib import Path
import datetime, hashlib, json, shutil, subprocess, tempfile, time

R = Path('/home/charl/moriarty-c2-opus55-restored-20261017')
J = Path('/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018')
C = R / 'composition-aggregate-check-cycle-20261018'
HEAD = '2f1b969ae4153c4f4f05ffe27a285a12e20f09cd'
TC = Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin')
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def now(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def write(name, value): (C/name).write_text(json.dumps(value, indent=2)+'\n')
def git(*args): return subprocess.check_output(['git', *args], cwd=J).decode().strip()
assert git('rev-parse','HEAD') == HEAD and not git('status','--porcelain=v1')
tree = git('rev-parse','HEAD^{tree}')
paths = subprocess.check_output(['git','ls-tree','-r','--name-only',HEAD,'crates','Cargo.toml','Cargo.lock','rust-toolchain.toml'],cwd=J).decode().splitlines()
pins=[]
for path in paths:
    raw=subprocess.check_output(['git','show',HEAD+':'+path],cwd=J)
    assert (J/path).read_bytes()==raw, path
    pins.append({'path':path,'sha256':hashlib.sha256(raw).hexdigest()})
write('SOURCE-PINS.json', {'head':HEAD,'tree':tree,'files':pins})
root=Path(tempfile.mkdtemp(prefix='doctorwatson-composition-aggregate-'))
for name in ['cwd','tmp','cargo-home']: (root/name).mkdir()
for name in ['registry','git']: (root/'cargo-home'/name).symlink_to(Path('/home/charl/.cargo')/name,target_is_directory=True)
env={'PATH':str(TC)+':/usr/bin:/bin','RUSTC':str(TC/'rustc'),'RUSTDOC':str(TC/'rustdoc'),'CARGO_HOME':str(root/'cargo-home'),'CARGO_TARGET_DIR':str(root/'target'),'CARGO_NET_OFFLINE':'true','CARGO_INCREMENTAL':'0','CARGO_CACHE_AUTO_CLEAN_FREQUENCY':'never','GIT_CONFIG_NOSYSTEM':'1','GIT_CONFIG_GLOBAL':'/dev/null','GIT_CONFIG_SYSTEM':'/dev/null','LANG':'C.UTF-8','TMPDIR':str(root/'tmp')}
configs=[]
for directory in [J,*J.parents,root/'cwd',*(root/'cwd').parents]:
    for filename in ['config','config.toml']:
        p=directory/'.cargo'/filename
        if str(p) not in configs:
            assert not p.exists(), str(p)
            configs.append(str(p))
argv=['/usr/bin/timeout','--signal=TERM','--kill-after=5s','90s',str(TC/'cargo'),'test','--manifest-path',str(J/'Cargo.toml'),'--frozen','-j1','-p','mori','--no-default-features','--features','core2-disjoint-draft','--test','disjoint_test_public','ordinary_expected_frame_mismatch_fails_and_retains_all_actual_reports','--','--exact','--test-threads=1']
write('CHECK-PLAN.json',{'recordedAt':now(),'head':HEAD,'tree':tree,'argv':argv,'environment':env,'ownedTemporaryRoot':str(root),'absentAncestorConfigs':configs,'toolPins':[{'path':str(p),'sha256':sha(p)} for p in [TC/'cargo',TC/'rustc',TC/'rustdoc',Path('/usr/bin/timeout')]],'scope':'One ordinary public composition comparison/aggregate regression; no SDK/native/key/proof/wallet/ledger call. Existing financial expectations unchanged; no financial allocation reset.','expected':'Exactly one selected test passes, old positive test filtered; complete failed aggregate and ten expanded reports equal frozen goldens.'})
result=None
try:
    started=time.monotonic()
    write('START.json',{'startedAt':now(),'head':HEAD,'tree':tree,'argv':argv})
    with (C/'STDOUT.log').open('wb') as out,(C/'STDERR.log').open('wb') as err:
        child=subprocess.Popen(argv,cwd=root/'cwd',env=env,stdin=subprocess.DEVNULL,stdout=out,stderr=err,start_new_session=True)
        write('CHILD.json',{'pid':child.pid,'session':child.pid})
        code=child.wait()
    remaining=[int(p) for p,s in (line.split() for line in subprocess.check_output(['ps','-eo','pid=,sid=']).decode().splitlines()) if int(s)==child.pid]
    result={'finishedAt':now(),'exitCode':code,'wallSeconds':time.monotonic()-started,'stdoutSha256':sha(C/'STDOUT.log'),'stderrSha256':sha(C/'STDERR.log'),'ownedSessionRemaining':remaining,'head':HEAD,'tree':tree,'sourcePinsUnchanged':all(sha(J/p['path'])==p['sha256'] for p in pins),'workingTreeClean':not git('status','--porcelain=v1')}
    write('RESULT.json',result)
    print(json.dumps(result),flush=True)
    assert code==0 and not remaining and result['sourcePinsUnchanged'] and result['workingTreeClean']
    assert '1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out' in (C/'STDOUT.log').read_text()
finally:
    shutil.rmtree(root)
    write('CLEANUP.json',{'recordedAt':now(),'ownedRoot':str(root),'existsAfter':root.exists(),'resultRecorded':result is not None})
