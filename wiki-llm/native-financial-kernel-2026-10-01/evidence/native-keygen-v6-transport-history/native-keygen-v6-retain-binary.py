"""Source-only bounded old-ELF retention; no process/network/Cargo/ELF execution."""
import os,sys,time,json,hashlib,resource,signal,shutil,ast
from pathlib import Path
base=Path('/home/charl/research/moriarty-signed-intent-2026-10-01')
source=base/'native-ledger-keygen-candidate';live=Path('/home/charl/research/moriarty-crypto-2026-09-30/target/debug/beta-native-ledger-consumer')
root=base/'native-keygen-v6-retained-v4-binary';archive=root/'beta-native-ledger-consumer'
G=1024**3;OLD_SIZE=624060592;OLD_SHA='034ed49b124c5b9118ee518c789ce276092567d4f991892ab396fdd6e6158688'
SOURCE_SHA='73a68f8577c271a236dd737d213742f2a7dcc8d3e5c5e1cab0688ef839adf8df'
FREEZE_SHA='d9c2d56e396e5720a26d1a07183602056d2aef1a9c98ca101a5103968a599d0b'
assert len(sys.argv)==3,'supervisor authorization path/hash required'
authorization=Path(sys.argv[1]);authorization_sha=sys.argv[2]
def sha(path):
 d=hashlib.sha256()
 with Path(path).open('rb') as f:
  for chunk in iter(lambda:f.read(1024*1024),b''):d.update(chunk)
 return d.hexdigest()
assert sha(authorization)==authorization_sha
allocation=json.loads(authorization.read_bytes())
assert set(allocation)=={'wrapper_sha256','candidate_source_sha256','result_freeze_sha256','terminal_v5_result_reviews','resource_votes','old_binary_archive'}
assert allocation['candidate_source_sha256']==SOURCE_SHA and allocation['result_freeze_sha256']==FREEZE_SHA
wrapper_path=base/'run-native-keygen-v6-bounded.py'
assert allocation['wrapper_sha256']==sha(wrapper_path)
wrapper_tree=ast.parse(wrapper_path.read_bytes())
retention_pins=[n.value.value for n in wrapper_tree.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='retention_pin' for t in n.targets) and isinstance(n.value,ast.Constant)]
assert len(retention_pins)==1 and sha(__file__)==retention_pins[0],'supervisor-pinned wrapper must bind retention source'
for family in ('terminal_v5_result_reviews','resource_votes'):
 entries=allocation[family];assert len(entries)==2 and {x['provider'] for x in entries}=={'Astra','Grok'}
 for x in entries:
  assert set(x)=={'provider','path','sha256','approved'} and x['approved'] is True and sha(x['path'])==x['sha256']
assert allocation['old_binary_archive']=={'path':str(archive),'sha256':OLD_SHA}
# Its own source identity is pinned by the separately frozen exact review package.
freeze_path=base/'NATIVE-KEYGEN-V6-SOURCE-FREEZE.json'
freeze_bytes=freeze_path.read_bytes();freeze_digest=hashlib.sha256(freeze_bytes).hexdigest()
self_pin=json.loads(freeze_bytes)['sha256']['native-keygen-v6-retain-binary.py'];assert sha(__file__)==self_pin
# Supervisor's resource vote hashes are administrative authority, not a financial acceptance flag.
def verify():
 assert sha(authorization)==authorization_sha and sha(freeze_path)==freeze_digest and sha(__file__)==self_pin
 assert sha(wrapper_path)==allocation['wrapper_sha256'],'wrapper identity changed'
 for f,h in json.loads(freeze_path.read_bytes())['sha256'].items():assert sha(base/f)==h,f
 assert sha(source/'SOURCE-HASHES.json')==SOURCE_SHA
 for f,h in json.loads((source/'SOURCE-HASHES.json').read_bytes())['sha256'].items():assert sha(source/f)==h,f
 for f,h in json.loads((source/'INPUT-SOURCE-HASHES.json').read_bytes())['sha256'].items():assert sha(f)==h,f
 result=base/'NATIVE-LEDGER-V5-RESULT-FREEZE.json';assert sha(result)==FREEZE_SHA
 for f,h in json.loads(result.read_bytes())['sha256'].items():assert sha(base/f)==h,f
 original=base/'native-ledger-consumer/manifest.json';assert sha(original)=='bf9db118385ecc328e1e20f9e10e9da08b845dc416477c41e8fe0eff62f5545f'
 m=json.loads(original.read_bytes())
 for f,h in m['sha256'].items():assert sha(base/'native-ledger-consumer'/f)==h,f
 for f,h in m['external_inputs_sha256'].items():assert sha(f)==h,f
 runtime=base/'NATIVE-LEDGER-RUNTIME-INPUTS-V5.json';assert sha(runtime)=='68afbf297ef0ca6cb7954d1e4ddab3065b7c89e4d8e5797e927cdf1d2cf4d425'
 r=json.loads(runtime.read_bytes());installed=Path(r['node_modules_root']);kernel=base/'kernel-prototype/node_modules'
 assert {str(p) for p in installed.rglob('*') if p.is_file()}==set(r['files_sha256'])
 assert {str(p) for p in kernel.rglob('*') if p.is_file()}==set(r['kernel_files_sha256'])
 for family in ('files_sha256','kernel_files_sha256'):
  for f,h in r[family].items():assert sha(f)==h,f
 assert kernel.is_symlink()==r['kernel_root_is_symlink'] and str(kernel.resolve())==r['kernel_root_resolved'] and kernel.resolve()==installed.resolve()
 for f,resolved in r['symlink_resolution'].items():assert Path(f).is_symlink() and str(Path(f).resolve())==resolved
 assert sha(base/'compiler-probe/runtime/package-lock.json')==r['package_lock_sha256']
 for family in ('terminal_v5_result_reviews','resource_votes'):
  for x in allocation[family]:assert sha(x['path'])==x['sha256']
 assert live.stat().st_size==OLD_SIZE and sha(live)==OLD_SHA,'actual reviewed old ELF changed'
verify()
assert not root.exists(),'exclusive fresh retention directory required; never reset failure'
receipt=base/'native-keygen-v6-retention-receipt.json';reservation=base/'native-keygen-v6-retention-attempt.json'
assert not receipt.exists()
assert shutil.disk_usage(base).free>=10*G
fd=os.open(reservation,os.O_WRONLY|os.O_CREAT|os.O_EXCL,0o644)
with os.fdopen(fd,'w') as f:
 json.dump({'scope':'old reviewed ELF copy only; no execution','authorization_sha256':authorization_sha,'self_sha256':self_pin,'source_freeze_sha256':freeze_digest,'state':'reserved; never delete/reset to retry'},f,indent=2);f.flush();os.fsync(f.fileno())
def sync_dir(path):
 fd=os.open(path,os.O_RDONLY|os.O_DIRECTORY)
 try:os.fsync(fd)
 finally:os.close(fd)
sync_dir(base)
resource.setrlimit(resource.RLIMIT_AS,(2*G,2*G));resource.setrlimit(resource.RLIMIT_CPU,(120,120));resource.setrlimit(resource.RLIMIT_FSIZE,(G,G))
def interrupted(signum,frame):raise InterruptedError('retention supervisor signal '+str(signum))
for sig in (signal.SIGINT,signal.SIGTERM,signal.SIGXCPU,signal.SIGXFSZ,signal.SIGALRM):signal.signal(sig,interrupted)
start=time.monotonic();cpu_start=time.process_time();copied=0;success=False;error=None;identity_ok=None
initial_free=shutil.disk_usage(base).free
signal.setitimer(signal.ITIMER_REAL,60) # Catchable absolute copy/posthash wall stop.
try:
 root.mkdir();sync_dir(base)
 d=hashlib.sha256()
 # This final-name file is PARTIAL until successful copy/hash/fsync receipt;
 # future wrapper still requires exact whole-file hash, never merely presence.
 with live.open('rb') as inp,archive.open('xb') as out:
  while True:
   if time.monotonic()-start>=60 or time.process_time()-cpu_start>=120:raise TimeoutError('retention wall/CPU ceiling')
   if shutil.disk_usage(base).free<10*G:raise OSError('retention free floor')
   if sum(p.stat().st_size for p in root.rglob('*') if p.is_file())>G:raise OSError('retention directory ceiling')
   block=inp.read(1024*1024)
   if not block:break
   copied+=len(block)
   if copied>OLD_SIZE:raise OSError('retention input grew')
   d.update(block);out.write(block)
  out.flush();os.fsync(out.fileno())
 if copied!=OLD_SIZE or d.hexdigest()!=OLD_SHA or sha(archive)!=OLD_SHA:raise OSError('retained ELF identity mismatch')
 os.chmod(archive,0o444);sync_dir(root);sync_dir(base)
 verify();identity_ok=True
 if shutil.disk_usage(base).free<10*G or sum(p.stat().st_size for p in root.rglob('*') if p.is_file())>G:raise OSError('final retention disk ceiling')
 if time.monotonic()-start>=60 or time.process_time()-cpu_start>=120:raise TimeoutError('final retention wall/CPU ceiling')
 success=True
except BaseException as e:
 error={'type':type(e).__name__,'message':str(e)}
 if time.monotonic()-start<60:
  try:verify();identity_ok=True
  except BaseException:identity_ok=None # Interrupted/unavailable identity is not a false mutation claim.
finally:
 # Preserve partial copy and consumed reservation; no unlink/reset or child process.
 signal.pthread_sigmask(signal.SIG_BLOCK,{signal.SIGINT,signal.SIGTERM,signal.SIGXCPU,signal.SIGXFSZ,signal.SIGALRM})
 signal.setitimer(signal.ITIMER_REAL,0)
 final_free=shutil.disk_usage(base).free
 record={'completed':success,'archive_path':str(archive),'bytes_copied':copied,'sha256':OLD_SHA if success else None,'source_identity_preserved':identity_ok,'authorization_sha256':authorization_sha,'source_freeze_sha256':freeze_digest,'fsync_done':success,'error':error,'initial_free_bytes':initial_free,'final_free_bytes':final_free,'elapsed_seconds':time.monotonic()-start,'cpu_seconds':time.process_time()-cpu_start,'limits':{'wall_seconds':60,'cpu_seconds':120,'address_space_bytes':2*G,'per_file_bytes':G,'total_retention_directory_bytes':G,'free_floor_bytes':10*G},'qualification':'retained reviewed ELF only; no process/network/Cargo/ELF execution'}
 fd=os.open(receipt,os.O_WRONLY|os.O_CREAT|os.O_EXCL,0o644)
 with os.fdopen(fd,'w') as f:json.dump(record,f,indent=2);f.write('\n');f.flush();os.fsync(f.fileno())
 sync_dir(base)
print('OLD_ELF_RETENTION_COMPLETE' if success else 'OLD_ELF_RETENTION_REFUSED');raise SystemExit(0 if success else 1)
