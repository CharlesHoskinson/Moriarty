# SOURCE PREPARATION: frozen actual prerequisites; execution requires fresh supervisor votes.
import os,sys,time,json,hashlib,subprocess,resource,signal,shutil
from pathlib import Path
base=Path('/home/charl/research/moriarty-signed-intent-2026-10-01')
source=base/'native-ledger-sorted-v9-r3-candidate';target=Path('/home/charl/research/moriarty-crypto-2026-09-30/target');cache=Path('/home/charl/.cargo')
G=1024**3;M=1024**2
# Actual v7 result identity; terminal result and resource reviews remain supervisor gates.
V7_RESULT_PIN='44c63a77861faca25737320db910fa404ae1081653918fceab1593e03b3e3389'
V8_INPUT_PIN='3f396e121e5ce1ff81ed210dd884c467f3e9a590aa9bd2651daa5f1328bb0149'
assert len(V7_RESULT_PIN)==64 and len(V8_INPUT_PIN)==64,'source-only draft; future result/integration pins unavailable'
assert len(sys.argv)==4,'prove|verify AUTHORIZATION AUTH_SHA required'
phase=sys.argv[1];assert phase in ('prove','verify'),'no build/fetch/keygen or chain mode'
source_pin='0179484a011c774bd5f6374200a28902d0873665ff098901574a679f9db189d7'
freeze_pin='d9c2d56e396e5720a26d1a07183602056d2aef1a9c98ca101a5103968a599d0b'
source_manifest=source/'SOURCE-HASHES.json'
def sha(path):
 h=hashlib.sha256()
 with Path(path).open('rb') as f:
  for chunk in iter(lambda:f.read(1024*1024),b''):h.update(chunk)
 return h.hexdigest()
def pinned(path,pin):
 assert sha(path)==pin,str(path)
 return json.loads(Path(path).read_bytes())
authorization_path=Path(sys.argv[2]);authorization_pin=sys.argv[3]
allocation=pinned(authorization_path,authorization_pin)
assert set(allocation)=={'wrapper_sha256','candidate_source_sha256','v5_result_freeze_sha256','v7_result_freeze_sha256','input_freeze_sha256','terminal_r3_result_reviews','resource_votes','proof_result_freeze','phase','source_freeze_sha256'}
assert allocation['phase']==phase
assert sha(base/'NATIVE-FINANCIAL-R3-PROOF-SOURCE-FREEZE.json')==allocation['source_freeze_sha256']
assert allocation['wrapper_sha256']==sha(__file__) and allocation['candidate_source_sha256']==source_pin
assert allocation['v5_result_freeze_sha256']==freeze_pin and allocation['v7_result_freeze_sha256']==V7_RESULT_PIN and allocation['input_freeze_sha256']==V8_INPUT_PIN
for family in ('terminal_r3_result_reviews','resource_votes'):
 entries=allocation[family];assert len(entries)==2 and {x['provider'] for x in entries}=={'Astra','Grok'}
 for item in entries:
  assert set(item)=={'provider','path','sha256','approved'} and item['approved'] is True
  assert sha(item['path'])==item['sha256']
# This supervisor freeze is FUTURE root authority; no child-produced manifest is accepted.
inputs_path=base/'NATIVE-FINANCIAL-R3-PROOF-INPUT-FREEZE.json'
inputs=pinned(inputs_path,V8_INPUT_PIN)
assert set(inputs)=={'sha256','binary','prove_config','v7_result_freeze'}
assert set(inputs['binary'])=={'path','sha256','bytes'}
binary_path=target/'debug/beta-native-ledger-consumer'
assert inputs['binary']['path']==str(binary_path)
binary_pin=inputs['binary']['sha256']
assert inputs['v7_result_freeze']=={'path':str(base/'NATIVE-KEYGEN-V7-RESULT-FREEZE.json'),'sha256':V7_RESULT_PIN}
# Required supervisor content: whole v7 result closure + receipts + keys/SRS,
# current full consumer/runtime/v5 closure, old v6 failures/archive and compiled ELF.
# Final source review must audit exact sha256 map completeness before root pins it.
def verify():
 assert sha(base/'NATIVE-FINANCIAL-R3-PROOF-SOURCE-FREEZE.json')==allocation['source_freeze_sha256']
 for n,h in json.loads((base/'NATIVE-FINANCIAL-R3-PROOF-SOURCE-FREEZE.json').read_bytes())['sha256'].items():assert sha(base/n)==h,n
 actual=pinned(base/'NATIVE-SORTED-V9-R3-EXECUTION-RESULT-FREEZE.json','0251b31fa5bba4f6b1a0cae91f4e98bd6a319333a55765b896d0cb3260848df1')
 assert actual['produced_binary']==inputs['binary']
 for n,h in actual['sha256'].items():assert sha(base/n)==h,n
 for mode in ('build','preflight'):
  a=json.loads((base/f'native-sorted-v9-r3-execution-{mode}-receipt.json').read_bytes())
  assert a['exit_code']==0 and a['stop_reason'] is None and a['source_identity_preserved'] is True and a['binary_identity_preserved'] is True and a['native_postconditions_passed'] is True and a['produced_binary_sha256']==binary_pin and a['source_manifest_sha256']==source_pin
 pre=json.loads((base/'native-sorted-v9-r3-execution-preflight-receipt.json').read_bytes())
 prep=json.loads((base/'native-sorted-v9-r3-execution-preflight/preparation.json').read_bytes())
 assert prep['native_check']=='successful actual Zkir::check' and prep['preimage_equal'] is True and prep['registered_vk_present'] is True and prep['proof_invocations']==0
 env=json.loads((base/'native-sorted-v9-r3-execution-preflight/envelope-preflight.json').read_bytes())
 assert env['proof_invocations']==0 and env['well_formed_checked'] is False and env['ledger_applied'] is False and env['ledger_accepted'] is False

 assert set(pre['output_artifacts'])=={str(p.relative_to(base/'native-sorted-v9-r3-execution-preflight')) for p in (base/'native-sorted-v9-r3-execution-preflight').rglob('*') if p.is_file()}
 for n,a in pre['output_artifacts'].items():assert sha(base/'native-sorted-v9-r3-execution-preflight'/n)==a['sha256'] and (base/'native-sorted-v9-r3-execution-preflight'/n).stat().st_size==a['bytes']

 assert sha(__file__)==allocation['wrapper_sha256'] and sha(authorization_path)==authorization_pin
 assert sha(inputs_path)==V8_INPUT_PIN and sha(source_manifest)==source_pin
 for f,h in inputs['sha256'].items():assert sha(f)==h,f
 v7=pinned(base/'NATIVE-KEYGEN-V7-RESULT-FREEZE.json',V7_RESULT_PIN)
 for f,h in v7['sha256'].items():assert sha(base/f)==h,f
 for f,h in json.loads(source_manifest.read_bytes())['sha256'].items():assert sha(source/f)==h,f
 inherited=pinned(base/'native-ledger-consumer/manifest.json','bf9db118385ecc328e1e20f9e10e9da08b845dc416477c41e8fe0eff62f5545f')
 for f,h in inherited['sha256'].items():assert sha(base/'native-ledger-consumer'/f)==h,f
 for f,h in inherited['external_inputs_sha256'].items():assert sha(f)==h,f
 for n in ('Cargo.toml','Cargo.lock'):assert sha(source/n)==sha(base/'native-ledger-consumer'/n)
 runtime=pinned(base/'NATIVE-LEDGER-RUNTIME-INPUTS-V5.json','68afbf297ef0ca6cb7954d1e4ddab3065b7c89e4d8e5797e927cdf1d2cf4d425')
 root=Path(runtime['node_modules_root']);kr=base/'kernel-prototype/node_modules'
 assert {str(p) for p in root.rglob('*') if p.is_file()}==set(runtime['files_sha256'])
 assert {str(p) for p in kr.rglob('*') if p.is_file()}==set(runtime['kernel_files_sha256'])
 for family in ('files_sha256','kernel_files_sha256'):
  for f,h in runtime[family].items():assert sha(f)==h,f
 assert kr.is_symlink()==runtime['kernel_root_is_symlink'] and str(kr.resolve())==runtime['kernel_root_resolved'] and kr.resolve()==root.resolve()
 for f,resolved in runtime['symlink_resolution'].items():assert Path(f).is_symlink() and str(Path(f).resolve())==resolved
 assert sha(base/'compiler-probe/runtime/package-lock.json')==runtime['package_lock_sha256']
 frozen=pinned(base/'NATIVE-LEDGER-V5-RESULT-FREEZE.json',freeze_pin)
 for f,h in frozen['sha256'].items():assert sha(base/f)==h,f
 assert sha(binary_path)==binary_pin and binary_path.stat().st_size==inputs['binary']['bytes']

 assert set(v7)=={'scope','sha256','produced_binary'} and len(v7['sha256'])==44
 assert v7['produced_binary']['sha256']=='9e1b153a17cd55969dd800fd47bc728923963093267cc8d23765f0a6ea484559'
 assert sha(base/'native-sorted-v9-r2-retained-v7-binary/beta-native-ledger-consumer')==v7['produced_binary']['sha256']
 for stage in ('build','srs','keygen'):
  r=json.loads((base/f'native-keygen-v7-{stage}-receipt.json').read_bytes())
  assert r['phase']==stage and r['exit_code']==0 and r['stop_reason'] is None and r['supervisor_error'] is None
  assert r['child_launched'] is True and r['source_identity_preserved'] is True and r['binary_identity_preserved'] is True
  assert r['source_manifest_sha256']=='73a68f8577c271a236dd737d213742f2a7dcc8d3e5c5e1cab0688ef839adf8df' and r['result_freeze_sha256']==freeze_pin and r['produced_binary_sha256']==v7['produced_binary']['sha256']
  assert r['supervisor_authorization_sha256']==sha(base/'NATIVE-KEYGEN-V7-SUPERVISOR-AUTHORIZATION.json')
  assert r['command']==json.loads((base/f'native-keygen-v7-{stage}-attempt.json').read_bytes())['command']
 assert json.loads((base/'native-keygen-v7-srs-receipt.json').read_bytes())['srs_sha256']=='4a9ef6c7c0619aab74eede44b13e753e3ba54508a02dd3b7106a949aabb73b74'
 acquired=json.loads((base/'native-keygen-v7-srs/acquisition.json').read_bytes())
 assert acquired['verified'] is True and acquired['stage']=='final_name_published' and acquired['bytes']==25166212 and acquired['bytes_written']==25166212
 assert acquired['sha256']=='4a9ef6c7c0619aab74eede44b13e753e3ba54508a02dd3b7106a949aabb73b74' and acquired['attempts']==1 and acquired['attempts_this_allocation']==1 and acquired['prior_consumed_transport_attempts']==1 and acquired['range_resume_fallback'] is False
 keys=json.loads((base/'native-keygen-v7-keys/keygen-success.json').read_bytes())
 assert keys['keygen_complete'] is True and keys['tagged_roundtrip_eof_identity'] is True and keys['k']==17
 assert keys['config_sha256']==sha(base/'native-keygen-v7-config.json') and keys['input_ir_sha256']=='c20c6e733500823c478eea885bebab5d62ba9958cfdd1ec8cf2ae89b32f9d7ae'
 assert keys['srs_sha256']==acquired['sha256'] and keys['srs_bytes']==25166212 and keys['authority_valid'] is None
 for n in ('proof_produced','well_formed_checked','ledger_applied','ledger_accepted'):assert keys[n] is False
 assert keys['resolver_registration_key_agreement']=='NotChecked; required in future actual finalized proof'
 assert len(keys['artifacts'])==3 and {x['name'] for x in keys['artifacts']}=={'pk.tagged','vk.tagged','ir.tagged'}
 assert {p.name for p in (base/'native-keygen-v7-keys').iterdir()}=={'pk.tagged','vk.tagged','ir.tagged','keygen-started.json','keygen-success.json'}
 for a in keys['artifacts']:
  p=base/'native-keygen-v7-keys'/a['name'];assert sha(p)==a['sha256'] and p.stat().st_size==a['bytes']
  assert json.loads((base/'native-keygen-v7-keygen-receipt.json').read_bytes())['key_artifacts'][a['name']]=={'sha256':a['sha256'],'bytes':a['bytes']}
 cfg=json.loads((base/'native-keygen-v7-config.json').read_bytes())
 assert cfg=={'ir':inputs['prove_config']['ir'],'srs':inputs['prove_config']['srs']}
 assert len(json.loads(source_manifest.read_bytes())['sha256'])==10
 assert len(json.loads((source/'INPUT-SOURCE-HASHES.json').read_bytes())['sha256'])==12
 assert len(inherited['sha256'])==39 and len(inherited['external_inputs_sha256'])==260
 assert len(runtime['files_sha256'])==240 and len(runtime['kernel_files_sha256'])==240 and len(frozen['sha256'])==34
 assert not (base/'native-keygen-v6-srs/bls_midnight_2p17').exists()
 assert not (base/'native-keygen-v6-build-attempt.json').exists() and not (base/'native-keygen-v6-keygen-attempt.json').exists()
 retained=json.loads((base/'native-keygen-v6-retention-receipt.json').read_bytes())
 assert retained['completed'] is True and retained['source_identity_preserved'] is True and retained['fsync_done'] is True and retained['bytes_copied']==624060592
 archive=base/'native-keygen-v6-retained-v4-binary/beta-native-ledger-consumer'
 assert archive.stat().st_size==624060592 and sha(archive)=='034ed49b124c5b9118ee518c789ce276092567d4f991892ab396fdd6e6158688'
 for family in ('terminal_r3_result_reviews','resource_votes'):
  for x in allocation[family]:assert sha(x['path'])==x['sha256']
verify()
# Exact schema is the unchanged native ProveConfig. Root supplies identities,
# wrapper pins them before launch and never takes paths/hashes from proof output.
pc=inputs['prove_config']
assert set(pc)=={'ir','pk','vk','srs','initial_contract','expected_contract','runtime','retained_preimage','fixture'}
for n in set(pc)-{'fixture'}:
 a=pc[n];assert set(a)=={'path','sha256'} and inputs['sha256'][a['path']]==a['sha256'] and sha(a['path'])==a['sha256']
baseline=json.loads((base/'native-ledger-v5-keyless-prepare-config.json').read_bytes())
for n in ('ir','initial_contract','expected_contract','runtime','retained_preimage','fixture'):assert pc[n]==baseline[n],'exact retained v5 producer projection required'
assert pc['fixture']=={'trust':'TRUSTED_GENESIS_PUBLIC_DEVELOPMENT_ONLY','network':'undeployed','block_seconds':1000000,'night_creation_seconds':0,'night_value':1000000000000,'fee_allowance':100000000000000000000,'ttl_seconds':1000300}
# construct() uses IrSource::load JSON; provider serializes this parsed IR internally.
assert pc['ir']=={'path':str(base/'kernel-prototype/output/zkir/pay.zkir'),'sha256':'c20c6e733500823c478eea885bebab5d62ba9958cfdd1ec8cf2ae89b32f9d7ae'}
for n,file in [('pk','pk.tagged'),('vk','vk.tagged')]:assert pc[n]['path']==str(base/'native-keygen-v7-keys'/file)
assert pc['srs']=={'path':str(base/'native-keygen-v7-srs/bls_midnight_2p17'),'sha256':'4a9ef6c7c0619aab74eede44b13e753e3ba54508a02dd3b7106a949aabb73b74'}
assert Path(pc['srs']['path']).stat().st_size==25166212
proof_dir=base/'native-financial-r3-proof';verify_dir=base/'native-financial-r3-verification'
out_dir=proof_dir if phase=='prove' else verify_dir
receipt=base/f'native-financial-r3-{phase}-receipt.json';reservation=base/f'native-financial-r3-{phase}-attempt.json'
config=base/f'native-financial-r3-{phase}-config.json'
assert not receipt.exists() and not reservation.exists() and not config.exists() and not out_dir.exists(),'exclusive allocation; no retry or overwrite'
if phase=='prove':
 assert allocation['proof_result_freeze'] is None,'proof cannot adopt existing results'
 cfg=pc;wall=600;cpu_limit=1200;rss_limit=4*G
else:
 proof_gate=allocation['proof_result_freeze'];assert set(proof_gate)=={'path','sha256'}
 assert proof_gate['path']==str(base/'NATIVE-FINANCIAL-R3-PROOF-RESULT-FREEZE.json')
 supervisor_proof=pinned(proof_gate['path'],proof_gate['sha256'])
 assert set(supervisor_proof)=={'sha256'}
 assert str(base/'native-financial-r3-prove-receipt.json') in supervisor_proof['sha256']
 for f,h in supervisor_proof['sha256'].items():assert sha(f)==h,f
 parent=json.loads((base/'native-financial-r3-prove-receipt.json').read_bytes())
 assert parent['exit_code']==0 and parent['stop_reason'] is None and parent['source_identity_preserved'] and parent['binary_identity_preserved']
 assert parent['produced_binary_sha256']==binary_pin and parent['input_freeze_sha256']==V8_INPUT_PIN and parent['v7_result_freeze_sha256']==V7_RESULT_PIN
 assert parent['native_postconditions_passed'] is True
 assert set(parent['output_artifacts'])=={str(p.relative_to(proof_dir)) for p in proof_dir.rglob('*') if p.is_file()}
 for n,a in parent['output_artifacts'].items():
  assert supervisor_proof['sha256'][str(proof_dir/n)]==a['sha256']
  assert sha(proof_dir/n)==a['sha256'] and (proof_dir/n).stat().st_size==a['bytes']
 cfg={n:pc[n] for n in ('vk','srs','expected_contract','fixture')}
 for n,file in [('proof','proof.raw'),('statement','statement.tagged'),('transaction','transaction.tagged'),('genesis','genesis.tagged')]:
  cfg[n]={'path':str(proof_dir/file),'sha256':parent['output_artifacts'][file]['sha256']}
 wall=120;cpu_limit=240;rss_limit=2*G
config_bytes=(json.dumps(cfg,indent=2)+'\n').encode();config_pin=hashlib.sha256(config_bytes).hexdigest()
cmd=['unshare','-Urn','--','env','RAYON_NUM_THREADS=2',str(binary_path),phase,str(config),config_pin,str(out_dir)]
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
reservation=base/f'native-financial-r3-{phase}-attempt.json'
fd=os.open(reservation,os.O_WRONLY|os.O_CREAT|os.O_EXCL,0o644)
with os.fdopen(fd,'w') as reserved:
 json.dump({'phase':phase,'command':cmd,'source_manifest_sha256':source_pin,'wrapper_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'state':'reserved before child launch; never delete to retry','unix_time':time.time()},reserved,indent=2);reserved.write('\n');reserved.flush();os.fsync(reserved.fileno())
directory_fd=os.open(base,os.O_RDONLY|os.O_DIRECTORY)
try:os.fsync(directory_fd)
finally:os.close(directory_fd)
start=time.monotonic();peak=0;maxcpu=0;stop=None;samples=[];nextdisk=0;hz=os.sysconf('SC_CLK_TCK');pages=os.sysconf('SC_PAGE_SIZE');p=None;exitcode=None;supervisor_error=None
try:
 with (base/f'native-financial-r3-{phase}.log').open('x') as log:
  if phase in ('prove','verify'):
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
    if size(proof_dir)+size(verify_dir)>512*M:stop='proof/verify output disk limit'
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
try:
 fd=os.open(base/f'native-financial-r3-{phase}.log',os.O_RDONLY)
 try:os.fsync(fd)
 finally:os.close(fd)
except OSError as error:stop=stop or 'terminal log fsync unavailable';supervisor_error=supervisor_error or {'type':type(error).__name__,'message':str(error)}
try:final=disk();final_rx=traffic()
except BaseException as error:
 final=initial;final_rx=initial_rx;stop=stop or 'final resource measurement unavailable';supervisor_error=supervisor_error or {'type':type(error).__name__,'message':str(error)}
if final['target_bytes']>10*G or final['free_bytes']<10*G or final['target_bytes']-initial['target_bytes']>2*G or final['cache_bytes']-initial['cache_bytes']>G:stop=stop or 'final disk limit'
if size(proof_dir)+size(verify_dir)>512*M:stop=stop or 'final output disk limit'
native_ok=False;artifacts={};binary_ok=False;identity_ok=False
try:
 verify();identity_ok=True
 assert sha(binary_path)==binary_pin;binary_ok=True
 assert sha(config)==config_pin,'parent-created closed config changed'
 if phase=='verify':
  assert sha(proof_gate['path'])==proof_gate['sha256']
  for f,h in supervisor_proof['sha256'].items():assert sha(f)==h,f
  assert set(parent['output_artifacts'])=={str(p.relative_to(proof_dir)) for p in proof_dir.rglob('*') if p.is_file()},'proof output set changed during verification'
  for n,a in parent['output_artifacts'].items():assert sha(proof_dir/n)==a['sha256'] and (proof_dir/n).stat().st_size==a['bytes']
 if out_dir.exists():
  for pth in out_dir.rglob('*'):
   if pth.is_file():artifacts[str(pth.relative_to(out_dir))]={'sha256':sha(pth),'bytes':pth.stat().st_size}
 if exitcode==0 and stop is None:
  result=json.loads((out_dir/('receipt.json' if phase=='prove' else 'independent-ledger-receipt.json')).read_bytes())
  assert result['result']=='Success' and result['proof_count']==1 and result['strictness']=='native default Real; proof-verifying enabled'
  assert result['runtime_ops']==293 and result['runtime_reads']==47
  fee=int(result['actual_native_fee_consumed']);allow=int(result['allow_fee_payment']);available=int(result['generationless_available']);remainder=int(result['dust_remainder'])
  assert 0<fee<=allow<=available and fee+remainder==available
  assert result['prestate_sha256']!=result['poststate_sha256']
  if phase=='prove':
   assert {'proof.raw','statement.tagged','skips.json','transaction.tagged','genesis.tagged','poststate.tagged','application-result.txt','replay-refusal.txt','receipt.json'}<=set(artifacts)
   assert result['statement_sha256']==artifacts['statement.tagged']['sha256'] and result['prestate_sha256']==artifacts['genesis.tagged']['sha256'] and result['poststate_sha256']==artifacts['poststate.tagged']['sha256']
   assert artifacts['proof.raw']['bytes']>0
  else:
   original=json.loads((proof_dir/'receipt.json').read_bytes());assert result==original,'independent actual acceptance differs'
   log=(base/f'native-financial-r3-{phase}.log').read_text()
   for marker in ('INDEPENDENT_CONDITIONAL_NATIVE_LEDGER_GOOD_AGAIN_OK','NATIVE_LEDGER_WELL_FORMED_REFUSAL absent ownership signature','STRICT_DECODER_REFUSAL','NATIVE_CRYPTO_REFUSAL'):assert marker in log
  native_ok=True
except (AssertionError,OSError,ValueError,TypeError,KeyError) as error:
 stop=stop or 'identity or native postcondition refused';supervisor_error=supervisor_error or {'type':type(error).__name__,'message':str(error)}
r={'phase':phase,'command':cmd,'source_manifest_sha256':source_pin,'v7_result_freeze_sha256':V7_RESULT_PIN,'input_freeze_sha256':V8_INPUT_PIN,'supervisor_authorization_sha256':authorization_pin,'config_sha256':config_pin,'exit_code':exitcode,'stop_reason':stop,'source_identity_preserved':identity_ok,'binary_identity_preserved':binary_ok,'produced_binary_sha256':binary_pin,'native_postconditions_passed':native_ok,'output_artifacts':artifacts,'attempt_reservation':str(reservation),'child_launched':p is not None,'supervisor_error':supervisor_error,'initial_disk':initial,'final_disk':final,'elapsed_seconds':time.monotonic()-start,'peak_sampled_group_rss_bytes':peak,'peak_sampled_group_cpu_seconds':maxcpu,'disk_samples':samples,'limits':{'wall_seconds':wall,'group_cpu_seconds':cpu_limit,'group_rss_bytes':rss_limit,'per_process_address_space_bytes':rss_limit,'per_file_bytes':2*G,'incremental_cache_bytes':G,'incremental_target_bytes':2*G,'total_target_bytes':10*G,'free_floor_bytes':10*G,'combined_proof_verify_output_bytes':512*M,'rayon_threads':2},'qualification':'conditional explicit trusted genesis, fixed kernel and exact production Beta/Core handoff; not generic compiler correspondence, live authenticated funding/deployment or Preview settlement','monitor_limitations':'group CPU/RSS sampled0.1s,disk5s; overshoot possible; Rayon2 is not a global hard thread cap'}
fd=os.open(receipt,os.O_WRONLY|os.O_CREAT|os.O_EXCL,0o644)
with os.fdopen(fd,'w') as output:json.dump(r,output,indent=2);output.write('\n');output.flush();os.fsync(output.fileno())
fd=os.open(base,os.O_RDONLY|os.O_DIRECTORY)
try:os.fsync(fd)
finally:os.close(fd)
print(json.dumps({k:r[k] for k in ('phase','exit_code','stop_reason','native_postconditions_passed')}))
raise SystemExit(exitcode if exitcode else (1 if stop else 0))
