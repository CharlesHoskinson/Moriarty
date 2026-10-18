from pathlib import Path
import datetime, hashlib, json, shutil, subprocess, tempfile, time

R=Path('/home/charl/moriarty-c2-opus55-restored-20261017')
J=Path('/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018')
C=R/'source6-max-rebate-check-cycle-20261018'
HEAD='8ee0a1f1ca98b09ba08221f0b1b52a1def2e8c65'
TC=Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin')
NODE=Path('/home/charl/.foreman/tools/fnm/node-versions/v24.21.0/installation/bin/node')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def write(name,x):
    p=C/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(x,indent=2)+'\n')
def git(*args):return subprocess.check_output(['git',*args],cwd=J).decode().strip()
assert git('rev-parse','HEAD')==HEAD and not git('status','--porcelain=v1')
tree=git('rev-parse','HEAD^{tree}')
fixture=J/'experiments/moriarty-language/regressions/fixtures/source6-max-rebate'
oracle=R/'core-positive-max-transfer-independent-oracle-20261018'
assert sha(oracle/'FREEZE.sha256')=='28671c70406c61f32870c5f40e5470c7fe7ef11a720a0d3aa8df84de0b3a77b0'
for line in (oracle/'FREEZE.sha256').read_text().splitlines():
    digest,path=line.split(maxsplit=1);path=path.lstrip('*');assert not Path(path).is_absolute() and '..' not in Path(path).parts;assert sha(oracle/path)==digest
for name,digest in json.loads((fixture/'FIXTURE-PINS.json').read_text())['files'].items():assert sha(fixture/name)==digest
paths=subprocess.check_output(['git','ls-tree','-r','--name-only',HEAD,'crates','Cargo.toml','Cargo.lock','rust-toolchain.toml','experiments/moriarty-language/src','experiments/moriarty-language/regressions/rust-source6-max-rebate.mjs','experiments/moriarty-language/regressions/fixtures/source6-max-rebate'],cwd=J).decode().splitlines();pins=[]
for path in paths:
    raw=subprocess.check_output(['git','show',HEAD+':'+path],cwd=J);assert (J/path).read_bytes()==raw,path;pins.append({'path':path,'sha256':hashlib.sha256(raw).hexdigest()})
write('SOURCE-PINS.json',{'head':HEAD,'tree':tree,'files':pins})
root=Path(tempfile.mkdtemp(prefix='doctorwatson-source6-max-'))
for name in ['cwd','tmp','cargo-home']:(root/name).mkdir()
for name in ['registry','git']:(root/'cargo-home'/name).symlink_to(Path('/home/charl/.cargo')/name,target_is_directory=True)
env={'PATH':str(TC)+':/usr/bin:/bin','RUSTC':str(TC/'rustc'),'RUSTDOC':str(TC/'rustdoc'),'CARGO_HOME':str(root/'cargo-home'),'CARGO_TARGET_DIR':str(root/'target'),'CARGO_NET_OFFLINE':'true','CARGO_INCREMENTAL':'0','CARGO_CACHE_AUTO_CLEAN_FREQUENCY':'never','GIT_CONFIG_NOSYSTEM':'1','GIT_CONFIG_GLOBAL':'/dev/null','GIT_CONFIG_SYSTEM':'/dev/null','LANG':'C.UTF-8','TMPDIR':str(root/'tmp'),'MORI_RUST_BIN':str(root/'target/debug/mori'),'MORI_SOURCE6_OBSERVATIONS':str(C/'observations')}
configs=[]
for directory in [J,*J.parents,root/'cwd',*(root/'cwd').parents]:
    for filename in ['config','config.toml']:
        p=directory/'.cargo'/filename
        if str(p) not in configs:assert not p.exists(),str(p);configs.append(str(p))
write('CHECK-PLAN.json',{'recordedAt':now(),'head':HEAD,'tree':tree,'environment':env,'ownedTemporaryRoot':str(root),'absentAncestorConfigs':configs,'toolPins':[{'path':str(p),'sha256':sha(p)} for p in [TC/'cargo',TC/'rustc',TC/'rustdoc',NODE,Path('/usr/bin/timeout')]],'independentOracleFreezeSha256':sha(oracle/'FREEZE.sha256'),'scope':'One new actual Rust beta→raw Source6→two real TS local preparation consumers; known admitted ordinary parser/checker/emitter/localpreparer only. No Midnight SDK/native/VM/proof/key/wallet/ledger or financial SDK allocation reset. Node is an inspected ELF binary, no Foreman runtime.','limits':'No Core compiler Artifact or full Core post/work/qualification oracle; all original profile/finance/PCD/target acceptance retained.'})
def run(name,cmd,bound):
    d=C/name;d.mkdir();argv=['/usr/bin/timeout','--signal=TERM','--kill-after=5s',str(bound)+'s',*cmd];write(name+'/START.json',{'startedAt':now(),'argv':argv,'cwd':str(root/'cwd'),'environment':env,'head':HEAD,'tree':tree});start=time.monotonic()
    with (d/'STDOUT.log').open('wb') as out,(d/'STDERR.log').open('wb') as err:
        child=subprocess.Popen(argv,cwd=root/'cwd',env=env,stdin=subprocess.DEVNULL,stdout=out,stderr=err,start_new_session=True);write(name+'/CHILD.json',{'pid':child.pid,'session':child.pid});code=child.wait()
    remaining=[int(p) for p,s in (line.split() for line in subprocess.check_output(['ps','-eo','pid=,sid=']).decode().splitlines()) if int(s)==child.pid]
    result={'finishedAt':now(),'exitCode':code,'wallSeconds':time.monotonic()-start,'stdoutSha256':sha(d/'STDOUT.log'),'stderrSha256':sha(d/'STDERR.log'),'ownedSessionRemaining':remaining};write(name+'/RESULT.json',result);assert not remaining;return result,d
build=None;test=None
try:
    build,d=run('rust-build',[str(TC/'cargo'),'build','--manifest-path',str(J/'Cargo.toml'),'--frozen','-j1','-p','mori','--no-default-features','--features','source6-expansion','--bin','mori'],90);print(json.dumps({'build':build}),flush=True);assert build['exitCode']==0
    binary_sha=sha(root/'target/debug/mori')
    write('RUST-BINARY.json',{'path':str(root/'target/debug/mori'),'sha256':binary_sha,'scope':'Actual source-only binary; removed with owned temporary target after test.'})
    test,d=run('node-consumers',[str(NODE),'--test',str(J/'experiments/moriarty-language/regressions/rust-source6-max-rebate.mjs')],35);print(json.dumps({'test':test}),flush=True);assert test['exitCode']==0
    assert git('rev-parse','HEAD')==HEAD and not git('status','--porcelain=v1')
    assert all(sha(J/p['path'])==p['sha256'] for p in pins) and sha(root/'target/debug/mori')==binary_sha
finally:
    shutil.rmtree(root);write('FINAL-RESULT.json',{'finishedAt':now(),'head':HEAD,'tree':tree,'build':build,'test':test,'ownedTemporaryRoot':str(root),'ownedRootExistsAfter':root.exists(),'scope':'One max/Qty/scalar/atoms/fee specimen through actual Beta→Source6→TS consumers. Complete rawoutputs retained before semantic assertions; only selected independent AST/effect/finance assertions qualify, no CoreArtifact/native/fullparent pass.'});write('CLEANUP.json',{'recordedAt':now(),'ownedRoot':str(root),'existsAfter':root.exists()})
