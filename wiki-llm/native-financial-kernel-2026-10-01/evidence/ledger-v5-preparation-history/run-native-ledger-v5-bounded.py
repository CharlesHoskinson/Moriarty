import os,sys,time,json,hashlib,subprocess,resource,signal,shutil
from pathlib import Path
base=Path('/home/charl/research/moriarty-signed-intent-2026-10-01')
source=base/'native-ledger-consumer'; target=Path('/home/charl/research/moriarty-crypto-2026-09-30/target'); cache=Path('/home/charl/.cargo')
phase=sys.argv[1] if len(sys.argv)==2 else ''
assert phase in ('handoff','prepare'),'exact production beta handoff then native preparation; no build allocation'
G=1024**3; M=1024**2
manifest=source/'manifest.json'; manifest_pin='bf9db118385ecc328e1e20f9e10e9da08b845dc416477c41e8fe0eff62f5545f'
assert hashlib.sha256(manifest.read_bytes()).hexdigest()==manifest_pin,'adapter source identity changed'
def verify():
 x=json.loads(manifest.read_bytes())
 for f,h in x['sha256'].items(): assert hashlib.sha256((source/f).read_bytes()).hexdigest()==h,f
 for f,h in x['external_inputs_sha256'].items(): assert hashlib.sha256(Path(f).read_bytes()).hexdigest()==h,f
runtime_inputs=base/'NATIVE-LEDGER-RUNTIME-INPUTS-V5.json'; runtime_inputs_pin='68afbf297ef0ca6cb7954d1e4ddab3065b7c89e4d8e5797e927cdf1d2cf4d425'
assert hashlib.sha256(runtime_inputs.read_bytes()).hexdigest()==runtime_inputs_pin
compiled_manifest_pin='a8a8825ca2b1cba66db6b6a83ce8ccdbcac6ff191bd184cc045388e542f8d8cb'
compiled_receipt_pin='df9cc75d34d9f24bd0ec94deab880a5db76d243fa76e77902829c613d095e03b'
compiled_binary_pin='034ed49b124c5b9118ee518c789ce276092567d4f991892ab396fdd6e6158688'
runtime_record=json.loads(runtime_inputs.read_bytes())
assert runtime_record['source_manifest_sha256']==manifest_pin
original_verify=verify
def verify():
 original_verify()
 assert hashlib.sha256(runtime_inputs.read_bytes()).hexdigest()==runtime_inputs_pin
 root=Path(runtime_record['node_modules_root'])
 assert {str(p) for p in root.rglob('*') if p.is_file()}==set(runtime_record['files_sha256']),'installed runtime file set changed'
 for f,h in runtime_record['files_sha256'].items():assert hashlib.sha256(Path(f).read_bytes()).hexdigest()==h,f
 for f,target_path in runtime_record['symlink_resolution'].items():assert Path(f).is_symlink() and str(Path(f).resolve())==target_path,f
 assert hashlib.sha256((base/'compiler-probe/runtime/package-lock.json').read_bytes()).hexdigest()==runtime_record['package_lock_sha256']
 kernel_root=base/'kernel-prototype/node_modules'
 assert kernel_root.is_symlink()==runtime_record['kernel_root_is_symlink'],'kernel runtime root symlink status changed'
 assert str(kernel_root.resolve())==runtime_record['kernel_root_resolved'],'kernel runtime root resolution changed'
 assert kernel_root.resolve()==root.resolve(),'kernel and installed runtime canonical roots differ'
 assert {str(p) for p in kernel_root.rglob('*') if p.is_file()}==set(runtime_record['kernel_files_sha256']),'kernel runtime file set changed'
 for f,h in runtime_record['kernel_files_sha256'].items():assert hashlib.sha256(Path(f).read_bytes()).hexdigest()==h,f
 if kernel_root.is_symlink():assert str(kernel_root) in runtime_record['symlink_resolution'],'kernel runtime root symlink not frozen'
 compiled_manifest=base/'NATIVE-LEDGER-V4-COMPILED-MANIFEST.json'
 assert hashlib.sha256(compiled_manifest.read_bytes()).hexdigest()==compiled_manifest_pin,'frozen compiled manifest changed'
 prior=json.loads(compiled_manifest.read_bytes());current=json.loads(manifest.read_bytes())
 assert prior['sha256']==current['sha256'],'compiled Rust/local source map changed'
 old_external=prior['external_inputs_sha256'];new_external=current['external_inputs_sha256']
 assert set(old_external)==set(new_external),'compiled/current external input keys changed'
 declared_helper_changes={str(base/'native-beta-handoff/handoff.ts'),str(base/'native-beta-handoff/SOURCE-HASHES.json')}
 actual_changes={f for f in old_external if old_external[f]!=new_external[f]}
 assert actual_changes==declared_helper_changes,'compiled input changes exceed exact Node helper repair'
 for f in set(old_external)-declared_helper_changes:assert old_external[f]==new_external[f],f
 old_build=base/'native-ledger-v4-build-receipt.json'
 assert hashlib.sha256(old_build.read_bytes()).hexdigest()==compiled_receipt_pin,'actual successful build receipt changed'
 built=json.loads(old_build.read_bytes())
 assert built['phase']=='build' and built['exit_code']==0 and built['stop_reason'] is None and built['source_identity_preserved'] is True and built['binary_identity_preserved'] is True,'actual v4 build failed or identity refused'
 assert built['adapter_manifest_sha256']==compiled_manifest_pin,'actual v4 compiled source pin mismatch'
 assert built['produced_binary_sha256']==compiled_binary_pin,'actual v4 compiled binary pin mismatch'
 assert Path(built['produced_binary_path'])==target/'debug/beta-native-ledger-consumer','actual v4 compiled binary path mismatch'


verify()
receipt=base/f'native-ledger-v5-{phase}-receipt.json'; assert not receipt.exists(),'one attempt per reviewed phase; preserve failed receipt'
export_dir=base/'native-ledger-v5-keyless-handoff'; prepare_dir=base/'native-ledger-v5-keyless-preparation'; config=base/'native-ledger-v5-keyless-prepare-config.json'
projection=base/'kernel-prototype/projection.json'; trace=base/'native-adapter-successor-v3/runtime-proof-data.json'
projection_pin='6afe4f734ab85f767eb2578a5b5a8699bfda81de360ce50b8fb9cb9cd82ee381'; trace_pin='2fb7f8343a5e773820abe9fe7d13f0ebe816a9ad043abb4d794f9450a80f000a'
primary=Path('/home/charl/Moriarty/.worktrees/moriarty-beta-20260930')
public_fixture=primary/'packages/moriarty-beta/examples/signed-intent/transfer-ecdsa-wallet'
handoff=['unshare','-Urn','--','node',str(base/'native-beta-handoff/cli.ts'),str(public_fixture/'program.mori'),'pay',str(public_fixture/'scenario.json'),str(public_fixture/'signature.json'),str(target/'debug/moriarty-midnight-crypto'),str(export_dir)]

if phase=='handoff':cmd=handoff; wall=60;cpu_limit=120;rss_limit=2*G
else:
 assert not config.exists(),'config must be created exclusively by this recipe'
 frozen=json.loads(manifest.read_bytes())['external_inputs_sha256']
 def specification(path):return {'path':str(path),'sha256':hashlib.sha256(path.read_bytes()).hexdigest()}
 cfg={'ir':specification(base/'kernel-prototype/output/zkir/pay.zkir'),'initial_contract':specification(export_dir/'initial-contract.tagged'),'expected_contract':specification(export_dir/'expected-contract.tagged'),'runtime':specification(export_dir/'runtime-native.json'),'retained_preimage':specification(base/'native-adapter-successor-v3/good.preimage'),'fixture':{'trust':'TRUSTED_GENESIS_PUBLIC_DEVELOPMENT_ONLY','network':'undeployed','block_seconds':1000000,'night_creation_seconds':0,'night_value':1000000000000,'fee_allowance':100000000000000000000,'ttl_seconds':1000300}}
 for k in ('ir','retained_preimage'):assert cfg[k]['sha256']==frozen[cfg[k]['path']],k
 config_bytes=(json.dumps(cfg,indent=2)+'\n').encode();config_pin=hashlib.sha256(config_bytes).hexdigest()
 cmd=['unshare','-Urn','--',str(target/'debug/beta-native-ledger-consumer'),'prepare',str(config),config_pin,str(prepare_dir)]
 wall=60;cpu_limit=120;rss_limit=2*G
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
binary_pin=None
if phase in ('handoff','prepare'):
 previous=json.loads((base/'native-ledger-v4-build-receipt.json').read_bytes())
 assert previous['exit_code']==0 and previous['stop_reason'] is None and previous['source_identity_preserved'],'build prerequisite refused'
 assert previous['adapter_manifest_sha256']==compiled_manifest_pin,'compiled source receipt mismatch'
 assert previous['produced_binary_sha256']==compiled_binary_pin,'compiled binary receipt hash mismatch'
 assert previous['binary_identity_preserved'] is True,'compiled binary receipt identity refused'
 binary_pin=previous['produced_binary_sha256']
 assert hashlib.sha256((target/'debug/beta-native-ledger-consumer').read_bytes()).hexdigest()==binary_pin,'produced binary changed'
 assert not export_dir.exists() if phase=='handoff' else not prepare_dir.exists(),'fresh outputs required'
def validate_handoff_receipt(record):
 assert record['status']=='NativeFixturePreparedUnqualified','handoff qualification changed'
 for field in ('ledger_accepted','proof_produced','well_formed_checked','ledger_applied'):assert record[field] is False,field
 assert record['authority_valid'] is None,'handoff authority falsely promoted'
 assert record['requiredPremises']==['canonical-intent-signature','snapshot-to-head','head-extension','atomic-ledger-compare-and-consume'],'handoff premises changed'
 assert record['unverifiedBindings']==['agreement-id','selected-program','asset-scale','authenticated-predecessor'],'handoff unverified bindings changed'
 assert record['trust']=='TRUSTED_GENESIS_PUBLIC_DEVELOPMENT_ONLY','handoff trust recipe changed'
 assert record['head_mapping']=={'betaPre':'h0','nativePre':'9e4601c9af94102208bd6bdea6693cbf5a9638f6d6b9fa716eae540443cc30e4','betaPost':'h1','nativePost':'4e87ddfda49bafabddab1a05cbdd0538735744a318dda80a211075ebb343b09e','authenticates_predecessor':False},'handoff fixed head mapping changed'
 assert record['head_mapping']['nativePre']!=record['head_mapping']['nativePost'],'head did not advance'
 frozen_inputs=json.loads(manifest.read_bytes())['external_inputs_sha256']
 for name,field in [('program.mori','source_sha256'),('scenario.json','scenario_sha256'),('signature.json','signature_artifact_sha256')]:assert record[field]==frozen_inputs[str(public_fixture/name)],field
 names={'initial-contract.tagged','expected-contract.tagged','runtime-native.json','registered-pay-operation.tagged'}
 items=record['artifacts'];assert len(items)==4 and {x['name'] for x in items}==names,'unexpected handoff artifact family'
 for item in items:
  assert set(item)=={'name','path','sha256'},'handoff artifact schema changed'
  assert Path(item['path'])==export_dir/item['name'],'handoff artifact path changed'
  assert hashlib.sha256((export_dir/item['name']).read_bytes()).hexdigest()==item['sha256'],item['name']
if phase=='prepare':
 previous_export=json.loads((base/'native-ledger-v5-handoff-receipt.json').read_bytes())
 assert previous_export['exit_code']==0 and previous_export['stop_reason'] is None and previous_export['source_identity_preserved'],'production beta handoff prerequisite refused'
 assert previous_export['adapter_manifest_sha256']==manifest_pin and previous_export['runtime_inputs_manifest_sha256']==runtime_inputs_pin,'handoff current source/runtime receipt mismatch'
 assert previous_export['binary_identity_preserved'] is True and previous_export['produced_binary_sha256']==compiled_binary_pin,'handoff compiled binary receipt mismatch'
 assert set(cfg)=={'ir','initial_contract','expected_contract','runtime','retained_preimage','fixture'},'no PK/VK/SRS permission'
 helper_receipt_bytes=(export_dir/'receipt.json').read_bytes()
 assert hashlib.sha256(helper_receipt_bytes).hexdigest()==previous_export['helper_receipt_sha256'],'export receipt changed after successful frozen observation'
 exported=json.loads(helper_receipt_bytes); validate_handoff_receipt(exported)
 for name,digest in previous_export['export_artifacts_sha256'].items():assert hashlib.sha256((export_dir/name).read_bytes()).hexdigest()==digest,name
 for name,count in previous_export['export_artifacts_bytes'].items():assert (export_dir/name).stat().st_size==count,name
 expected_paths={'ir':Path('/home/charl/research/moriarty-signed-intent-2026-10-01/kernel-prototype/output/zkir/pay.zkir'),'initial_contract':export_dir/'initial-contract.tagged','expected_contract':export_dir/'expected-contract.tagged','runtime':export_dir/'runtime-native.json','retained_preimage':base/'native-adapter-successor-v3/good.preimage'}
 for k,a in ((k,cfg[k]) for k in expected_paths):
  assert Path(a['path'])==expected_paths[k],k
  assert hashlib.sha256(expected_paths[k].read_bytes()).hexdigest()==a['sha256'],k
  if k in ('ir','retained_preimage'):assert a['sha256']==json.loads(manifest.read_bytes())['external_inputs_sha256'][str(expected_paths[k])],k
  else:assert a['sha256']==next(item['sha256'] for item in exported['artifacts'] if item['name']==expected_paths[k].name),k
 assert cfg['fixture']=={'trust':'TRUSTED_GENESIS_PUBLIC_DEVELOPMENT_ONLY','network':'undeployed','block_seconds':1000000,'night_creation_seconds':0,'night_value':1000000000000,'fee_allowance':100000000000000000000,'ttl_seconds':1000300},'fixture recipe changed' 
 helper_config_bytes=(export_dir/'prepare-config.json').read_bytes()
 assert hashlib.sha256(helper_config_bytes).hexdigest()==previous_export['handoff_prepare_config_sha256'],'handoff config bytes changed'
 assert hashlib.sha256(helper_config_bytes).hexdigest()==exported['prepare_config_sha256'],'handoff config receipt mismatch'
 assert json.loads(helper_config_bytes)==cfg,'handoff config differs from independent supervisor recipe'

def limits():
 resource.setrlimit(resource.RLIMIT_AS,(rss_limit,rss_limit)); resource.setrlimit(resource.RLIMIT_CPU,(cpu_limit,cpu_limit));resource.setrlimit(resource.RLIMIT_FSIZE,(2*G,2*G))
 signal.pthread_sigmask(signal.SIG_SETMASK,launch_saved_mask) # Child restores original mask before exec.
def interrupted(signum,frame):raise KeyboardInterrupt(f'supervisor signal {signum}')
for sig in (signal.SIGINT,signal.SIGTERM):signal.signal(sig,interrupted)
reservation=base/f'native-ledger-v5-{phase}-attempt.json'
fd=os.open(reservation,os.O_WRONLY|os.O_CREAT|os.O_EXCL,0o644)
with os.fdopen(fd,'w') as reserved:
 json.dump({'phase':phase,'command':cmd,'adapter_manifest_sha256':manifest_pin,'wrapper_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'state':'reserved before child launch; never delete to retry','unix_time':time.time()},reserved,indent=2);reserved.write('\n');reserved.flush();os.fsync(reserved.fileno())
directory_fd=os.open(base,os.O_RDONLY|os.O_DIRECTORY)
try:os.fsync(directory_fd)
finally:os.close(directory_fd)
start=time.monotonic();peak=0;maxcpu=0;stop=None;samples=[];nextdisk=0;hz=os.sysconf('SC_CLK_TCK');pages=os.sysconf('SC_PAGE_SIZE');p=None;exitcode=None;supervisor_error=None
try:
 with (base/f'native-ledger-v5-{phase}.log').open('x') as log:
  if phase=='prepare':
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
    if state['target_bytes']>8*G or state['free_bytes']<10*G:stop='total target/free disk limit'
    if state['cache_bytes']-initial['cache_bytes']>G:stop='incremental source/cache disk limit'
    if state['target_bytes']-initial['target_bytes']>2*G:stop='incremental target disk limit'
    if phase in ('handoff','prepare') and size(export_dir)+size(prepare_dir)>32*M:stop='preparation output disk limit'
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
# Helpers reuse only the actual successful v4 executable, frozen before launch.
binary_path=target/'debug/beta-native-ledger-consumer'
binary_identity_ok=None;binary_identity_error=None
binary_artifact_status='prerequisite_v4_binary_frozen'
export_artifacts=None;helper_receipt_pin=None
handoff_config_pin=None;export_artifact_sizes=None
if phase=='handoff' and exitcode==0 and stop is None:
 try:
  helper_receipt_bytes=(export_dir/'receipt.json').read_bytes();exported=json.loads(helper_receipt_bytes)
  validate_handoff_receipt(exported)
  export_artifacts={item['name']:item['sha256'] for item in exported['artifacts']}
  export_artifact_sizes={item['name']:(export_dir/item['name']).stat().st_size for item in exported['artifacts']}
  helper_receipt_pin=hashlib.sha256(helper_receipt_bytes).hexdigest()
  helper_config_bytes=(export_dir/'prepare-config.json').read_bytes()
  handoff_config_pin=hashlib.sha256(helper_config_bytes).hexdigest()
  assert handoff_config_pin==exported['prepare_config_sha256'],'actual handoff config receipt mismatch'
  frozen=json.loads(manifest.read_bytes())['external_inputs_sha256']
  def fixed_input(path):return {'path':str(path),'sha256':frozen[str(path)]}
  def observed_output(name):return {'path':str(export_dir/name),'sha256':export_artifacts[name]}
  expected_config={'ir':fixed_input(base/'kernel-prototype/output/zkir/pay.zkir'),'initial_contract':observed_output('initial-contract.tagged'),'expected_contract':observed_output('expected-contract.tagged'),'runtime':observed_output('runtime-native.json'),'retained_preimage':fixed_input(base/'native-adapter-successor-v3/good.preimage'),'fixture':{'trust':'TRUSTED_GENESIS_PUBLIC_DEVELOPMENT_ONLY','network':'undeployed','block_seconds':1000000,'night_creation_seconds':0,'night_value':1000000000000,'fee_allowance':100000000000000000000,'ttl_seconds':1000300}}
  assert json.loads(helper_config_bytes)==expected_config,'handoff config differs from closed independent supervisor recipe'

 except (AssertionError,OSError,ValueError,KeyError,TypeError) as error:
  stop='actual production beta handoff postcondition refused';supervisor_error={'type':type(error).__name__,'message':str(error)}

if phase=='prepare' and exitcode==0 and stop is None:
 try:
  result=json.loads((prepare_dir/'preparation.json').read_bytes())
  assert result['proof_invocations']==0 and result['preimage_equal'] is True and result['registered_vk_present'] is False and result['native_check']=='successful actual Zkir::check'
  assert hashlib.sha256(config.read_bytes()).hexdigest()==config_pin,'config changed'
 except (AssertionError,OSError,ValueError,KeyError,TypeError) as error:
  stop='preparation postcondition refused';supervisor_error={'type':type(error).__name__,'message':str(error)}
if phase in ('handoff','prepare') and size(export_dir)+size(prepare_dir)>32*M:stop=stop or 'final preparation output disk limit'
source_identity_error=None
try:
 verify()
 identity_ok=True
except (AssertionError,OSError,ValueError,TypeError,KeyError) as error:
 identity_ok=False;stop=stop or 'source identity changed during phase'
 source_identity_error={'type':type(error).__name__,'message':str(error)}
# Reused actual v4 binary is checked before launch above and again here,
# even if the helper failed. Source/runtime identity remains separate.
if binary_pin is not None:
 try:
  assert hashlib.sha256(binary_path.read_bytes()).hexdigest()==binary_pin,'binary identity changed during phase'
  binary_identity_ok=True
 except (AssertionError,OSError) as error:
  binary_identity_ok=False;binary_artifact_status='binary_identity_refused'
  stop=stop or 'produced binary identity changed during phase'
  binary_identity_error={'type':type(error).__name__,'message':str(error)}
r={'command':cmd,'adapter_manifest_sha256':manifest_pin,'compiled_source_manifest_sha256':compiled_manifest_pin,'compiled_build_receipt_sha256':compiled_receipt_pin,'produced_binary_path':str(binary_path),'produced_binary_sha256':binary_pin,'initial_disk':initial,'final_disk':final,'exit_code':exitcode,'stop_reason':stop,'source_identity_preserved':identity_ok,'source_identity_error':source_identity_error,'binary_artifact_status':binary_artifact_status,'binary_identity_preserved':binary_identity_ok,'binary_identity_error':binary_identity_error,'child_launched':p is not None,'supervisor_error':supervisor_error,'attempt_reservation':str(reservation),'elapsed_seconds':time.monotonic()-start,'peak_sampled_group_rss_bytes':peak,'peak_sampled_group_cpu_seconds':maxcpu,'global_nonloopback_received_byte_delta':final_rx-initial_rx,'disk_samples':samples,'limits':{'wall_seconds':wall,'group_cpu_seconds':cpu_limit,'group_rss_bytes':rss_limit,'per_process_address_space_bytes':rss_limit,'incremental_source_cache_bytes':G,'incremental_target_bytes':2*G,'total_target_bytes':8*G,'free_floor_bytes':10*G,'disk_sample_interval_seconds':5,'memory_cpu_network_sample_interval_seconds':.1,'cargo_jobs':2,'rayon_threads':2},'monitor_limitations':'RSS/CPU/traffic sampled at0.1s and disk at5s; stop/receipt records overshoot. Traffic is global nonloopback RX, a conservative upper bound including other host activity, not isolated Cargo body measurement. Build/check run in verified no-network user namespace.','qualification':'production beta handoff or no-key typed native preparation only; no SRS/keygen/proof/well_formed/apply/Preview acceptance','helper_receipt_sha256':helper_receipt_pin,'export_artifacts_sha256':export_artifacts,'export_artifacts_bytes':export_artifact_sizes,'handoff_prepare_config_sha256':handoff_config_pin,'runtime_inputs_manifest_sha256':runtime_inputs_pin,'phase':phase,'config_sha256':config_pin if phase=='prepare' else None,'preparation_output_limit_bytes':32*M}
fd=os.open(receipt,os.O_WRONLY|os.O_CREAT|os.O_EXCL,0o644)
with os.fdopen(fd,'w') as recorded:
 json.dump(r,recorded,indent=2);recorded.write('\n');recorded.flush();os.fsync(recorded.fileno())
print(json.dumps({k:r[k] for k in ('exit_code','stop_reason','elapsed_seconds','source_identity_preserved')}));raise SystemExit(exitcode if exitcode else (1 if stop else 0))
