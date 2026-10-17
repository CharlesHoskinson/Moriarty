import hashlib,json,subprocess,time
from pathlib import Path
from datetime import datetime,timezone
R=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017');HERE=Path(__file__).resolve().parent;OUT=HERE/'attempt-01'
S=R/'101-structural-sharing-oracle';source=S/'proposal/crates/mori-core/tests/sherlock_original_009_structural_sharing.rs'
lib=R/'74-compiler-correction-review-packet/harness-clarification-02/prospective-run/libmori_core.rlib'
D=R/'24-compiler-qa/proposed-run/core-target/debug/deps';rustc='/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc'
selector='uireq009_shared_leaf4_and_distinct_leaf4_occurrences_both_fold_to8'
def pin(p):
 p=Path(p);b=p.read_bytes();return dict(path=str(p),bytes=len(b),sha256=hashlib.sha256(b).hexdigest())
def save(name,value): (OUT/name).write_text(json.dumps(value,indent=2)+'\n')
inputs=json.loads((R/'99-public-grouping-actual-preparation/FROZEN-PINS.json').read_text())['existing_normal_dependencies_tools']
freeze=json.loads((S/'SOURCE-FREEZE.json').read_text())
for f in freeze['files']:inputs.append(dict(f,path=str(S/f['path'])))
inputs += [dict(path=str(lib),bytes=31369804,sha256='909320dbd4e47f4c546ebfcb56e6cd007a4516b26ceee1e43bb94e3be4217561')]
for path in [R/'106-structural-sharing-source-review/REVIEW.json',R/'106-structural-sharing-eligibility-clarification-01/CLARIFICATION.json',Path(__file__)]:inputs.append(pin(path))
def check():
 for f in inputs:
  actual=pin(f['path']);assert all(actual[k]==f[k] for k in ('path','bytes','sha256')),f['path']
 if (OUT/'test-bin').exists():assert pin(OUT/'test-bin')==sealed,'compiled test binary changed'
def phase(name,argv):
 check();save(name+'-PREEXEC.json',dict(argv=argv,cwd=str(OUT),environment=env));start=time.monotonic();cp=subprocess.run(argv,cwd=OUT,env=env,stdin=subprocess.DEVNULL,capture_output=True);elapsed=time.monotonic()-start
 (OUT/(name+'.stdout')).write_bytes(cp.stdout);(OUT/(name+'.stderr')).write_bytes(cp.stderr);save(name+'-RESULT.json',dict(argv=argv,exit_code=cp.returncode,elapsed_seconds=elapsed,stdout=pin(OUT/(name+'.stdout')),stderr=pin(OUT/(name+'.stderr'))));assert cp.returncode==0,(name,cp.returncode)
 return cp.stdout.decode()
OUT.mkdir(exist_ok=False);(OUT/'tmp').mkdir();env=dict(PATH='/usr/bin:/bin',LANG='C.UTF-8',TMPDIR=str(OUT/'tmp'));sealed=None
save('ROOT-SELECTION.json',dict(utc=datetime.now(timezone.utc).isoformat(),scope='Exact R106 eligibility plus explicit correction; two fixed A Machine runs only',source=pin(source),library=pin(lib),limits_seconds=dict(compile=30,list=10,test=10),native_authority=0,no_retry=True))
check();save('INPUTS.json',inputs)
argv=['/usr/bin/timeout','--signal=KILL','30s',rustc,'--edition=2024','--test',str(source),'--extern','mori_core='+str(lib),'-L','dependency='+str(D),'--emit=link,dep-info='+str(OUT/'test.d'),'-o',str(OUT/'test-bin'),'-C','debuginfo=0']
# Before link there is no sealed binary; save actual status even on any failure.
# The binary first appears during compilation, so seal it immediately thereafter.
check();save('01-compile-PREEXEC.json',dict(argv=argv,cwd=str(OUT),environment=env));start=time.monotonic();cp=subprocess.run(argv,cwd=OUT,env=env,stdin=subprocess.DEVNULL,capture_output=True);elapsed=time.monotonic()-start
(OUT/'01-compile.stdout').write_bytes(cp.stdout);(OUT/'01-compile.stderr').write_bytes(cp.stderr);save('01-compile-RESULT.json',dict(argv=argv,exit_code=cp.returncode,elapsed_seconds=elapsed,stdout=pin(OUT/'01-compile.stdout'),stderr=pin(OUT/'01-compile.stderr')));assert cp.returncode==0,cp.returncode
sealed=pin(OUT/'test-bin');save('SEALED-TEST-BINARY.json',sealed);check()
listed=phase('02-list',['/usr/bin/timeout','--signal=KILL','10s',str(OUT/'test-bin'),'--list']);assert [s.removesuffix(': test') for s in listed.splitlines() if s.endswith(': test')]==[selector] and listed.rstrip().endswith('1 test, 0 benchmarks'),listed
output=phase('03-exact',['/usr/bin/timeout','--signal=KILL','10s',str(OUT/'test-bin'),'--exact',selector,'--test-threads=1']);assert 'test '+selector+' ... ok' in output and '1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out' in output,output
check();save('RESULT.json',dict(status='pass',selector=selector,source=pin(source),inputs_unchanged=len(inputs),independent_expectations=pin(S/'EXPECTED.json'),qualification='Original009 fixed shared/distinct pair; no full independent prefix-state oracle/general009/native claim',native_authority=0));print('PASS exact original009 shared/distinct pair; '+str(len(inputs))+' pins unchanged')
