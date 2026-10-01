import os,sys,time,json,hashlib,subprocess,resource,signal,shutil
from pathlib import Path
base=Path('/home/charl/research/moriarty-signed-intent-2026-10-01')
source=base/'native-ledger-keygen-candidate';target=Path('/home/charl/research/moriarty-crypto-2026-09-30/target');cache=Path('/home/charl/.cargo')
assert len(sys.argv)==4,'phase supervisor-authorization-file supervisor-authorization-SHA required'
phase=sys.argv[1];assert phase in ('build','srs','keygen'),'no proof/WF/apply mode'
G=1024**3;M=1024**2
source_manifest=source/'SOURCE-HASHES.json';source_pin='73a68f8577c271a236dd737d213742f2a7dcc8d3e5c5e1cab0688ef839adf8df'
freeze=base/'NATIVE-LEDGER-V5-RESULT-FREEZE.json';freeze_pin='d9c2d56e396e5720a26d1a07183602056d2aef1a9c98ca101a5103968a599d0b'
SRS_SHA='4a9ef6c7c0619aab74eede44b13e753e3ba54508a02dd3b7106a949aabb73b74';SRS_BYTES=25166212
IR=base/'kernel-prototype/output/zkir/pay.zkir';IR_SHA='c20c6e733500823c478eea885bebab5d62ba9958cfdd1ec8cf2ae89b32f9d7ae'
old_binary_pin='034ed49b124c5b9118ee518c789ce276092567d4f991892ab396fdd6e6158688'
fetch_source=base/'native-keygen-v7-srs-fetch.py';fetch_pin='99bcbdc85a7f5ca09f9ef603f1fa88449537be295f6c9c9a8934d44eab250f84'
retention_pin='89c372825ad99e0ad5a54fbacbf9a475e7f427269d464033178ac4ddb18340dd'
history_pins={'/home/charl/research/moriarty-signed-intent-2026-10-01/run-native-keygen-v6-bounded.py': 'd1014784628ec2539035a99ffc0769b5e0d1eab15bfbdb38844bb97b33193c0a', '/home/charl/research/moriarty-signed-intent-2026-10-01/native-keygen-v6-srs-fetch.py': '5769d9c23c03dfbcdf8cfe27de59191780a64492006cfcbc8a31fb3f505893e9', '/home/charl/research/moriarty-signed-intent-2026-10-01/native-keygen-v6-retain-binary.py': '89c372825ad99e0ad5a54fbacbf9a475e7f427269d464033178ac4ddb18340dd', '/home/charl/research/moriarty-signed-intent-2026-10-01/NATIVE-KEYGEN-V6-SOURCE-FREEZE.json': '76dd7dfe69df23e0abee9b17db44a1fd08b0fda849a11dbead575a036d270b44', '/home/charl/research/moriarty-signed-intent-2026-10-01/NATIVE-KEYGEN-V6-SUPERVISOR-AUTHORIZATION.json': '3c2c6c58a7995b6cf0dc0363f27ce45dc0ce29461ba19550bc83e1af7af36c60', '/home/charl/research/moriarty-signed-intent-2026-10-01/native-keygen-v6-retention-attempt.json': '7e767dbffecd7c44113a08236d2e0da1af605d66f4957248a56851b840387fd0', '/home/charl/research/moriarty-signed-intent-2026-10-01/native-keygen-v6-retention-receipt.json': 'f12d09e24166f57bcf61f48761f5082294a3e5ab5889d05a3b6f65f8357574db', '/home/charl/research/moriarty-signed-intent-2026-10-01/native-keygen-v6-srs-attempt.json': '03e2405eefde7341be3fa470b124fe156207af4c2367b9893d13d7a8a677656b', '/home/charl/research/moriarty-signed-intent-2026-10-01/native-keygen-v6-srs-receipt.json': '2211b63c3dd2d620feca4b3d70bef3ad2dd3550aaced52ed8cd67984920d1e4c', '/home/charl/research/moriarty-signed-intent-2026-10-01/native-keygen-v6-srs.log': '7465e3fed9169e95c219b7a43cbaf1a72eb16a6d6a171c0608ee715f96a21aef', '/home/charl/research/moriarty-signed-intent-2026-10-01/native-keygen-v6-srs/acquisition-failed.json': 'ec784034bd6c008552a53adf5e2545bbed9b0bf7602f297d5adf3f1687a837f4', '/home/charl/research/moriarty-signed-intent-2026-10-01/native-keygen-v6-srs/srs.part': '242070c2be0e98541837ece08f9c3179bb4a532ea921918e071c19cbc225b2c5'}
def sha(path):return hashlib.sha256(Path(path).read_bytes()).hexdigest()
authorization_path=Path(sys.argv[2]);authorization_pin=sys.argv[3]
assert sha(authorization_path)==authorization_pin,'supervisor authorization identity mismatch'
allocation=json.loads(authorization_path.read_bytes())
assert set(allocation)=={'wrapper_sha256','candidate_source_sha256','result_freeze_sha256','terminal_v5_result_reviews','resource_votes','old_binary_archive'},'closed supervisor allocation required'
assert allocation['wrapper_sha256']==sha(__file__) and allocation['candidate_source_sha256']==source_pin and allocation['result_freeze_sha256']==freeze_pin
# Supervisor records administrative votes, not a financial/proof acceptance premise.
for family in ('terminal_v5_result_reviews','resource_votes'):
 entries=allocation[family];assert len(entries)==2 and {x['provider'] for x in entries}=={'Astra','Grok'}
 for item in entries:
  assert set(item)=={'provider','path','sha256','approved'},'closed vote identity'
  assert item['approved'] is True and sha(item['path'])==item['sha256'],'missing/refused or changed terminal vote'
archive=allocation['old_binary_archive'];assert set(archive)=={'path','sha256'}
assert Path(archive['path'])==base/'native-keygen-v6-retained-v4-binary/beta-native-ledger-consumer','exact retained binary path required'
assert Path(archive['path']).resolve()!= (target/'debug/beta-native-ledger-consumer').resolve(),'archive must be distinct'
assert Path(archive['path']).stat().st_size==624060592,'exact old ELF archive bytes required'
assert archive['sha256']==old_binary_pin and sha(archive['path'])==old_binary_pin,'old reviewed executable must be retained before future build'
def verify():
 assert sha(authorization_path)==authorization_pin and sha(source_manifest)==source_pin and sha(freeze)==freeze_pin and sha(fetch_source)==fetch_pin
 assert sha(__file__)==allocation['wrapper_sha256'],'supervisor wrapper identity changed'
 for f,h in history_pins.items():assert sha(f)==h,f
 for f,h in json.loads((base/'NATIVE-KEYGEN-V6-SOURCE-FREEZE.json').read_bytes())['sha256'].items():assert sha(base/f)==h,f
 prior_allocation=json.loads((base/'NATIVE-KEYGEN-V6-SUPERVISOR-AUTHORIZATION.json').read_bytes())
 for family in ('terminal_v5_result_reviews','resource_votes'):
  for item in prior_allocation[family]:assert sha(item['path'])==item['sha256'],item['path']
 assert sha(base/'NATIVE-KEYGEN-V6-SUPERVISOR-AUTHORIZATION.json')=='3c2c6c58a7995b6cf0dc0363f27ce45dc0ce29461ba19550bc83e1af7af36c60'
 assert not (base/'native-keygen-v6-srs/bls_midnight_2p17').exists(),'old failed acquisition must not be silently promoted'
 assert not (base/'native-keygen-v6-build-attempt.json').exists() and not (base/'native-keygen-v6-keygen-attempt.json').exists(),'unexpected old phase execution'
 assert sha(base/'native-keygen-v6-retain-binary.py')==retention_pin,'retention helper source changed'
 retained=json.loads((base/'native-keygen-v6-retention-receipt.json').read_bytes())
 assert retained['completed'] is True and retained['source_identity_preserved'] is True and retained['fsync_done'] is True and retained['sha256']==old_binary_pin and retained['bytes_copied']==624060592 and retained['authorization_sha256']=='3c2c6c58a7995b6cf0dc0363f27ce45dc0ce29461ba19550bc83e1af7af36c60','retention prerequisite refused'
 inherited=base/'native-ledger-consumer/manifest.json';assert sha(inherited)=='bf9db118385ecc328e1e20f9e10e9da08b845dc416477c41e8fe0eff62f5545f'
 inherited_map=json.loads(inherited.read_bytes())
 for f,h in inherited_map['sha256'].items():assert sha(base/'native-ledger-consumer'/f)==h,f
 for f,h in inherited_map['external_inputs_sha256'].items():assert sha(f)==h,f
 for name in ('Cargo.toml','Cargo.lock'):assert sha(source/name)==sha(base/'native-ledger-consumer'/name),'keygen dependency graph changed'
 runtime_path=base/'NATIVE-LEDGER-RUNTIME-INPUTS-V5.json';assert sha(runtime_path)=='68afbf297ef0ca6cb7954d1e4ddab3065b7c89e4d8e5797e927cdf1d2cf4d425'
 runtime=json.loads(runtime_path.read_bytes());root=Path(runtime['node_modules_root']);kernel_root=base/'kernel-prototype/node_modules'
 assert {str(p) for p in root.rglob('*') if p.is_file()}==set(runtime['files_sha256'])
 assert {str(p) for p in kernel_root.rglob('*') if p.is_file()}==set(runtime['kernel_files_sha256'])
 for family in ('files_sha256','kernel_files_sha256'):
  for f,h in runtime[family].items():assert sha(f)==h,f
 assert kernel_root.is_symlink()==runtime['kernel_root_is_symlink'] and str(kernel_root.resolve())==runtime['kernel_root_resolved'] and kernel_root.resolve()==root.resolve()
 for f,resolved in runtime['symlink_resolution'].items():assert Path(f).is_symlink() and str(Path(f).resolve())==resolved,f
 assert sha(base/'compiler-probe/runtime/package-lock.json')==runtime['package_lock_sha256']

 for f,h in json.loads(source_manifest.read_bytes())['sha256'].items():assert sha(source/f)==h,f
 for f,h in json.loads((source/'INPUT-SOURCE-HASHES.json').read_bytes())['sha256'].items():assert sha(f)==h,f
 for f,h in json.loads(freeze.read_bytes())['sha256'].items():assert sha(base/f)==h,f
 assert sha(IR)==IR_SHA and sha(archive['path'])==old_binary_pin
 assert Path(archive['path']).stat().st_size==624060592,'old ELF archive size changed'
 for family in ('terminal_v5_result_reviews','resource_votes'):
  for item in allocation[family]:assert sha(item['path'])==item['sha256']
verify()
prepared=json.loads((base/'native-ledger-v5-keyless-preparation/preparation.json').read_bytes())
assert prepared['native_check']=='successful actual Zkir::check' and prepared['preimage_equal'] is True and prepared['proof_invocations']==0 and prepared['registered_vk_present'] is False
receipt=base/f'native-keygen-v7-{phase}-receipt.json';assert not receipt.exists(),'preserve consumed phase'
reservation=base/f'native-keygen-v7-{phase}-attempt.json'
srs_dir=base/'native-keygen-v7-srs';keys_dir=base/'native-keygen-v7-keys';config=base/'native-keygen-v7-config.json'
binary_path=target/'debug/beta-native-ledger-consumer';binary_pin=None
if phase=='build':
 assert sha(binary_path)==old_binary_pin,'old reviewed binary changed before new candidate build'
 cmd=['unshare','-Urn','--','env','CARGO_BUILD_JOBS=2','RAYON_NUM_THREADS=2','CARGO_NET_OFFLINE=true',f'CARGO_TARGET_DIR={target}','cargo','build','--locked','--offline','--manifest-path','Cargo.toml','--bin','beta-native-ledger-consumer'];wall=120;cpu_limit=240;rss_limit=4*G
else:
 built=json.loads((base/'native-keygen-v7-build-receipt.json').read_bytes())
 assert built['exit_code']==0 and built['stop_reason'] is None and built['source_identity_preserved'] and built['binary_identity_preserved'] and built['source_manifest_sha256']==source_pin
 binary_pin=built['produced_binary_sha256'];assert sha(binary_path)==binary_pin
 if phase=='srs':
  assert not srs_dir.exists(),'one fresh full acquisition only; no v6 resume'
  cmd=[sys.executable,str(fetch_source),str(srs_dir)];wall=600;cpu_limit=120;rss_limit=2*G
 else:
  acquired=json.loads((base/'native-keygen-v7-srs-receipt.json').read_bytes())
  assert acquired['exit_code']==0 and acquired['stop_reason'] is None and acquired['source_identity_preserved'] and acquired['binary_identity_preserved'] and acquired['source_manifest_sha256']==source_pin and acquired['produced_binary_sha256']==binary_pin and acquired['srs_sha256']==SRS_SHA
  assert sha(srs_dir/'bls_midnight_2p17')==SRS_SHA and (srs_dir/'bls_midnight_2p17').stat().st_size==SRS_BYTES
  assert not keys_dir.exists() and not config.exists(),'fresh config/key outputs required'
  cfg={'ir':{'path':str(IR),'sha256':IR_SHA},'srs':{'path':str(srs_dir/'bls_midnight_2p17'),'sha256':SRS_SHA}}
  config_bytes=(json.dumps(cfg,indent=2)+'\n').encode();config_pin=hashlib.sha256(config_bytes).hexdigest()
  cmd=['unshare','-Urn','--','env','RAYON_NUM_THREADS=2',str(binary_path),'keygen',str(config),config_pin,str(keys_dir)];wall=300;cpu_limit=600;rss_limit=4*G

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
initial=disk();initial_rx=traffic();assert initial['target_bytes']<=10*G and initial['free_bytes']>=10*G
def limits():
 resource.setrlimit(resource.RLIMIT_AS,(rss_limit,rss_limit)); resource.setrlimit(resource.RLIMIT_CPU,(cpu_limit,cpu_limit));resource.setrlimit(resource.RLIMIT_FSIZE,(2*G,2*G))
 signal.pthread_sigmask(signal.SIG_SETMASK,launch_saved_mask) # Child restores original mask before exec.
def interrupted(signum,frame):raise KeyboardInterrupt(f'supervisor signal {signum}')
for sig in (signal.SIGINT,signal.SIGTERM):signal.signal(sig,interrupted)
reservation=base/f'native-keygen-v7-{phase}-attempt.json'
fd=os.open(reservation,os.O_WRONLY|os.O_CREAT|os.O_EXCL,0o644)
with os.fdopen(fd,'w') as reserved:
 json.dump({'phase':phase,'command':cmd,'source_manifest_sha256':source_pin,'wrapper_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'state':'reserved before child launch; never delete to retry','unix_time':time.time()},reserved,indent=2);reserved.write('\n');reserved.flush();os.fsync(reserved.fileno())
directory_fd=os.open(base,os.O_RDONLY|os.O_DIRECTORY)
try:os.fsync(directory_fd)
finally:os.close(directory_fd)
start=time.monotonic();peak=0;maxcpu=0;stop=None;samples=[];nextdisk=0;hz=os.sysconf('SC_CLK_TCK');pages=os.sysconf('SC_PAGE_SIZE');p=None;exitcode=None;supervisor_error=None
try:
 with (base/f'native-keygen-v7-{phase}.log').open('x') as log:
  if phase=='keygen':
   with config.open('xb') as created:created.write(config_bytes);created.flush();os.fsync(created.fileno())
   directory_fd=os.open(base,os.O_RDONLY|os.O_DIRECTORY)
   try:os.fsync(directory_fd)
   finally:os.close(directory_fd)
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
    if state['target_bytes']>10*G or state['free_bytes']<10*G:stop='total target/free disk limit'
    if state['cache_bytes']-initial['cache_bytes']>G:stop='incremental source/cache disk limit'
    if state['target_bytes']-initial['target_bytes']>2*G:stop='incremental target disk limit'
    if size(srs_dir)+size(keys_dir)>512*M:stop='SRS/key output disk limit'
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
if final['target_bytes']>10*G or final['free_bytes']<10*G or final['target_bytes']-initial['target_bytes']>2*G or final['cache_bytes']-initial['cache_bytes']>G:stop=stop or 'final disk limit'
if size(srs_dir)+size(keys_dir)>512*M:stop=stop or 'final output disk limit'
binary_identity_ok=None;srs_pin=None;key_records=None
if phase=='build' and exitcode==0 and stop is None:
 try:
  assert binary_path.is_file() and binary_path.stat().st_mode&0o111,'produced executable missing'
  binary_pin=sha(binary_path)
 except (AssertionError,OSError) as error:stop='produced executable unavailable';supervisor_error={'type':type(error).__name__,'message':str(error)}
if phase=='srs' and exitcode==0 and stop is None:
 try:
  assert (srs_dir/'bls_midnight_2p17').stat().st_size==SRS_BYTES and sha(srs_dir/'bls_midnight_2p17')==SRS_SHA
  acquisition=json.loads((srs_dir/'acquisition.json').read_bytes());assert acquisition['verified'] is True and acquisition['bytes']==SRS_BYTES and acquisition['sha256']==SRS_SHA and acquisition['attempts']==1 and acquisition['attempts_this_allocation']==1 and acquisition['prior_consumed_transport_attempts']==1 and acquisition['stage']=='final_name_published' and acquisition['bytes_written']==SRS_BYTES
  srs_pin=SRS_SHA
 except (AssertionError,OSError,ValueError,KeyError) as error:stop='actual SRS identity refused';supervisor_error={'type':type(error).__name__,'message':str(error)}
if phase=='keygen' and exitcode==0 and stop is None:
 try:
  result=json.loads((keys_dir/'keygen-success.json').read_bytes())
  assert result['keygen_complete'] is True and result['tagged_roundtrip_eof_identity'] is True and result['k']==17 and result['input_ir_sha256']==IR_SHA and result['srs_sha256']==SRS_SHA and result['srs_bytes']==SRS_BYTES
  assert result['config_sha256']==config_pin and sha(config)==config_pin
  assert result['authority_valid'] is None
  for f in ('proof_produced','well_formed_checked','ledger_applied','ledger_accepted'):assert result[f] is False,f
  assert result['resolver_registration_key_agreement']=='NotChecked; required in future actual finalized proof'
  assert len(result['artifacts'])==3 and {x['name'] for x in result['artifacts']}=={'pk.tagged','vk.tagged','ir.tagged'}
  key_records={}
  for item in result['artifacts']:
   path=keys_dir/item['name'];assert path.stat().st_size==item['bytes'] and sha(path)==item['sha256'];key_records[item['name']]={'sha256':item['sha256'],'bytes':item['bytes']}
 except (AssertionError,OSError,ValueError,KeyError,TypeError) as error:stop='actual keygen postcondition refused';supervisor_error={'type':type(error).__name__,'message':str(error)}
try:verify();identity_ok=True
except (AssertionError,OSError,ValueError,TypeError,KeyError) as error:identity_ok=False;stop=stop or 'source/frozen result identity changed';supervisor_error=supervisor_error or {'type':type(error).__name__,'message':str(error)}
if binary_pin is not None:
 try:assert sha(binary_path)==binary_pin;binary_identity_ok=True
 except (AssertionError,OSError):binary_identity_ok=False;stop=stop or 'produced binary identity changed'
r={'phase':phase,'command':cmd,'source_manifest_sha256':source_pin,'result_freeze_sha256':freeze_pin,'supervisor_authorization_sha256':authorization_pin,'exit_code':exitcode,'stop_reason':stop,'source_identity_preserved':identity_ok,'produced_binary_sha256':binary_pin,'binary_identity_preserved':binary_identity_ok,'srs_sha256':srs_pin,'key_artifacts':key_records,'attempt_reservation':str(reservation),'child_launched':p is not None,'supervisor_error':supervisor_error,'initial_disk':initial,'final_disk':final,'elapsed_seconds':time.monotonic()-start,'peak_sampled_group_rss_bytes':peak,'peak_sampled_group_cpu_seconds':maxcpu,'global_nonloopback_received_byte_delta':final_rx-initial_rx,'disk_samples':samples,'limits':{'wall_seconds':wall,'group_cpu_seconds':cpu_limit,'group_rss_bytes':rss_limit,'per_process_address_space_bytes':rss_limit,'per_file_bytes':2*G,'incremental_source_cache_bytes':G,'incremental_target_bytes':2*G,'total_target_bytes':10*G,'free_floor_bytes':10*G,'SRS_key_output_bytes':512*M,'cargo_jobs':2,'rayon_threads':2},'qualification':'offline build/new full SRS request/keygen only; no proof/WF/apply/Preview','monitor_limitations':'RSS/CPU0.1s,disk5s sampled; overshoot possible. Rayon env limits Rayon pool, not every native thread. Network receipt redacts HTTP headers; global RX includes unrelatedhostactivity.'}
fd=os.open(receipt,os.O_WRONLY|os.O_CREAT|os.O_EXCL,0o644)
with os.fdopen(fd,'w') as out:json.dump(r,out,indent=2);out.write('\n');out.flush();os.fsync(out.fileno())
print(json.dumps({k:r[k] for k in ('phase','exit_code','stop_reason','source_identity_preserved')}));raise SystemExit(exitcode if exitcode else (1 if stop else 0))
