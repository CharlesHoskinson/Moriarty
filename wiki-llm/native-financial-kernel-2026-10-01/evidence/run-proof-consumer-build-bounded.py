import os,sys,time,json,hashlib,subprocess,resource,signal,shutil
from pathlib import Path
base=Path('/home/charl/research/moriarty-signed-intent-2026-10-01')
source=base/'native-proof-consumer'; target=Path('/home/charl/research/moriarty-crypto-2026-09-30/target'); cache=Path('/home/charl/.cargo')
phase=sys.argv[1] if len(sys.argv)==2 else ''
assert phase == 'build','choose exactly build; no native consumer execution'
G=1024**3; M=1024**2
manifest=source/'manifest.json'; manifest_pin='f8fa2cd7fb85a9417946536b55e331d1deb77f88839acb6d584e9d306696e1f7'
assert hashlib.sha256(manifest.read_bytes()).hexdigest()==manifest_pin,'adapter source identity changed'
def verify():
 x=json.loads(manifest.read_bytes())
 for f,h in x['sha256'].items(): assert hashlib.sha256((source/f).read_bytes()).hexdigest()==h,f
 for f,h in x['external_inputs_sha256'].items(): assert hashlib.sha256(Path(f).read_bytes()).hexdigest()==h,f
verify()
receipt=base/f'proof-consumer-{phase}-receipt.json'; assert not receipt.exists(),'one attempt per reviewed phase; preserve failed receipt'
build=['unshare','-Urn','--','env','CARGO_BUILD_JOBS=2','RAYON_NUM_THREADS=2','CARGO_NET_OFFLINE=true',f'CARGO_TARGET_DIR={target}','cargo','build','--locked','--offline','--manifest-path','Cargo.toml','--bin','beta-native-proof-consumer']
cmd=build
wall=600; cpu_limit=1200; rss_limit=4*G
def size(path):
 total=0
 for file in path.rglob('*'):
  try:
   if file.is_file():total+=file.stat().st_size
  except FileNotFoundError:pass # A deleted file occupies no current apparent bytes.
 return total
def disk():return {'target_bytes':size(target),'cache_bytes':size(cache),'free_bytes':shutil.disk_usage(target).free}
def traffic():
 total=0
 for line in Path('/proc/net/dev').read_text().splitlines()[2:]:
  name,fields=line.split(':',1)
  if name.strip()!='lo':total+=int(fields.split()[0])
 return total
initial=disk(); initial_rx=traffic(); assert initial['target_bytes']<=8*G and initial['free_bytes']>=10*G
if phase=='build':
 f=json.loads((base/'adapter-v3-check-receipt.json').read_text()); assert f['exit_code']==0 and f['stop_reason'] is None and f['source_identity_preserved'],'native adapter check failed; no consumer build'
 metadata=json.loads((source/'metadata-receipt.json').read_text()); assert metadata['exit_code']==0,'offline source graph unresolved; no consumer build'
binary_pin=None # Build-only; binary is never executed.
def limits():
 resource.setrlimit(resource.RLIMIT_AS,(rss_limit,rss_limit)); resource.setrlimit(resource.RLIMIT_CPU,(cpu_limit,cpu_limit));resource.setrlimit(resource.RLIMIT_FSIZE,(2*G,2*G))
 signal.pthread_sigmask(signal.SIG_SETMASK,launch_saved_mask) # Child restores original mask before exec.
def interrupted(signum,frame):raise KeyboardInterrupt(f'supervisor signal {signum}')
for sig in (signal.SIGINT,signal.SIGTERM):signal.signal(sig,interrupted)
reservation=base/f'proof-consumer-{phase}-attempt.json'
fd=os.open(reservation,os.O_WRONLY|os.O_CREAT|os.O_EXCL,0o644)
with os.fdopen(fd,'w') as reserved:
 json.dump({'phase':phase,'command':cmd,'adapter_manifest_sha256':manifest_pin,'wrapper_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'state':'reserved before child launch; never delete to retry','unix_time':time.time()},reserved,indent=2);reserved.write('\n');reserved.flush();os.fsync(reserved.fileno())
directory_fd=os.open(base,os.O_RDONLY|os.O_DIRECTORY)
try:os.fsync(directory_fd)
finally:os.close(directory_fd)
start=time.monotonic();peak=0;maxcpu=0;stop=None;samples=[];nextdisk=0;hz=os.sysconf('SC_CLK_TCK');pages=os.sysconf('SC_PAGE_SIZE');p=None;exitcode=None;supervisor_error=None
try:
 with (base/f'proof-consumer-{phase}.log').open('x') as log:
  launch_saved_mask=signal.pthread_sigmask(signal.SIG_BLOCK,{signal.SIGINT,signal.SIGTERM})
  try:
   p=subprocess.Popen(cmd,cwd=source,stdout=log,stderr=subprocess.STDOUT,start_new_session=True,preexec_fn=limits)
  finally:
   # Pending interrupts may raise here; p is already owned by outer cleanup.
   signal.pthread_sigmask(signal.SIG_SETMASK,launch_saved_mask)
  while p.poll() is None:
   elapsed=time.monotonic()-start;rss=0;cpu=0
   for stat in Path('/proc').glob('[0-9]*/stat'):
    try:
     v=stat.read_text().rsplit(')',1)[1].split()
     if int(v[2])==p.pid:rss+=int(v[21])*pages;cpu+=(int(v[11])+int(v[12])+int(v[13])+int(v[14]))/hz
    except (OSError,ValueError):pass
   peak=max(peak,rss);maxcpu=max(maxcpu,cpu)
   if elapsed>=wall:stop='wall limit'
   if cpu>=cpu_limit:stop='group CPU limit'
   if rss>rss_limit:stop='group RSS limit'
   if elapsed>=nextdisk:
    state=disk();samples.append({'seconds':round(elapsed,3),**state,'group_rss_bytes':rss,'group_cpu_seconds':cpu});nextdisk=elapsed+5
    if state['target_bytes']>8*G or state['free_bytes']<10*G:stop='total target/free disk limit'
    if state['cache_bytes']-initial['cache_bytes']>G:stop='incremental source/cache disk limit'
    if state['target_bytes']-initial['target_bytes']>2*G:stop='incremental target disk limit'
   if stop:break
   time.sleep(.1)
except BaseException as error:
 supervisor_error={'type':type(error).__name__,'message':str(error)};stop=stop or 'supervisor exception or signal'
finally:
 signal.pthread_sigmask(signal.SIG_BLOCK,{signal.SIGINT,signal.SIGTERM})
 # Cleanup runs for all catchable monitor/log/spawn/wait exceptions and SIGINT/TERM.
 # Uncatchable host SIGKILL/power loss cannot execute finally; durable reservation remains.
 if p is not None:
  try:os.killpg(p.pid,signal.SIGKILL)
  except ProcessLookupError:pass
  exitcode=p.wait()
try:final=disk();final_rx=traffic()
except BaseException as error:
 final=initial;final_rx=initial_rx;stop=stop or 'final resource measurement unavailable';supervisor_error=supervisor_error or {'type':type(error).__name__,'message':str(error)}

if final['target_bytes']>8*G or final['free_bytes']<10*G or final['target_bytes']-initial['target_bytes']>2*G or final['cache_bytes']-initial['cache_bytes']>G:stop=stop or 'final disk limit'
try:verify();identity_ok=True
except (AssertionError,OSError):identity_ok=False;stop=stop or 'source identity changed during phase'
r={'command':cmd,'adapter_manifest_sha256':manifest_pin,'checker_binary_sha256':binary_pin,'initial_disk':initial,'final_disk':final,'exit_code':exitcode,'stop_reason':stop,'source_identity_preserved':identity_ok,'child_launched':p is not None,'supervisor_error':supervisor_error,'attempt_reservation':str(reservation),'elapsed_seconds':time.monotonic()-start,'peak_sampled_group_rss_bytes':peak,'peak_sampled_group_cpu_seconds':maxcpu,'global_nonloopback_received_byte_delta':final_rx-initial_rx,'disk_samples':samples,'limits':{'wall_seconds':wall,'group_cpu_seconds':cpu_limit,'group_rss_bytes':rss_limit,'per_process_address_space_bytes':rss_limit,'incremental_source_cache_bytes':G,'incremental_target_bytes':2*G,'total_target_bytes':8*G,'free_floor_bytes':10*G,'disk_sample_interval_seconds':5,'memory_cpu_network_sample_interval_seconds':.1,'cargo_jobs':2,'rayon_threads':2},'monitor_limitations':'RSS/CPU/traffic sampled at0.1s and disk at5s; stop/receipt records overshoot. Traffic is global nonloopback RX, a conservative upper bound including other host activity, not isolated Cargo body measurement. Build/check run in verified no-network user namespace.','qualification':'new native proof consumer compilation only; binary is not executed; no SRS/keygen/proof/ledger/Preview acceptance'}
fd=os.open(receipt,os.O_WRONLY|os.O_CREAT|os.O_EXCL,0o644)
with os.fdopen(fd,'w') as recorded:
 json.dump(r,recorded,indent=2);recorded.write('\n');recorded.flush();os.fsync(recorded.fileno())
print(json.dumps({k:r[k] for k in ('exit_code','stop_reason','elapsed_seconds','source_identity_preserved')}));raise SystemExit(exitcode if exitcode else (1 if stop else 0))
