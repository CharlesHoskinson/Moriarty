import os,time,json,hashlib,subprocess,resource,signal,shutil
from pathlib import Path
repo=Path('/home/charl/Moriarty/.worktrees/moriarty-beta-20260930'); target=Path('/home/charl/research/moriarty-crypto-2026-09-30/target'); out=Path('/home/charl/research/moriarty-signed-intent-2026-10-01')
pins={'src/financial_transfer.rs':'f72e902b336582ded4b720efbfa362e30039d6f79c07d4b1165ef8317b692de4','tests/financial_transfer.rs':'41e632534b815f984cd72bdb0f49153ed3602bfe95b34a8bb19bc61305406f6e','src/lib.rs':'396f0e86dcd72a9e4dfd7684df872b88c9f4415cab42a96779c11e3622a7d041','Cargo.toml':'15dedfcb94a3d8ffe8290e30e4c9858e200b9a10682de3db759f93e402204388','Cargo.lock':'53731faef893a205c4c96933845d72253e3b55277b3bdefc6cf7c355d8b46bab','fixtures/moriarty.json':'0761254e1166e5ba3ec3f6059d9fa2540d54cedf4accfee7107ba9ecd73095d5'}
assert not (out/'native-run-receipt.json').exists(),'One attempt only'
for f,h in pins.items(): assert hashlib.sha256((repo/'experiments/midnight-crypto'/f).read_bytes()).hexdigest()==h,f
G=1024**3
# Apparent bytes conservatively include hard links more than once.
def disk():
 total=sum(p.stat().st_size for p in target.rglob('*') if p.is_file());free=shutil.disk_usage(target).free
 return total,free
initial=disk();assert initial[0]<=8*G and initial[1]>=10*G
cmd=['cargo','test','--offline','--locked','--manifest-path','experiments/midnight-crypto/Cargo.toml','--features','native-proof','--test','financial_transfer','native_transfer_proof_bounded_k10','--','--ignored','--exact','--nocapture']
def limits():
 resource.setrlimit(resource.RLIMIT_AS,(8*G,8*G));resource.setrlimit(resource.RLIMIT_CPU,(600,600))
env={**os.environ,'CARGO_BUILD_JOBS':'2','CARGO_TARGET_DIR':str(target),'RAYON_NUM_THREADS':'2'}
start=time.monotonic();peak=0;maxcpu=0;stop=None;samples=[];nextdisk=0;hz=os.sysconf('SC_CLK_TCK');pages=os.sysconf('SC_PAGE_SIZE')
with (out/'native-run.log').open('w') as log:
 p=subprocess.Popen(cmd,cwd=repo,env=env,stdout=log,stderr=subprocess.STDOUT,start_new_session=True,preexec_fn=limits)
 while p.poll() is None:
  elapsed=time.monotonic()-start;rss=0;cpu=0
  for stat in Path('/proc').glob('[0-9]*/stat'):
   try:
    v=stat.read_text().rsplit(')',1)[1].split()
    if int(v[2])==p.pid: rss+=int(v[21])*pages;cpu+=(int(v[11])+int(v[12])+int(v[13])+int(v[14]))/hz
   except (OSError,ValueError):pass
  peak=max(peak,rss);maxcpu=max(maxcpu,cpu)
  if elapsed>=600:stop='wall limit'
  if cpu>=600:stop='group CPU limit'
  if rss>8*G:stop='group RSS limit'
  if elapsed>=nextdisk:
   used,free=disk();samples.append({'seconds':round(elapsed,3),'target_bytes':used,'free_bytes':free,'group_rss_bytes':rss,'group_cpu_seconds':cpu});nextdisk=elapsed+5
   if used>8*G or free<10*G:stop='disk limit'
  if stop:
   os.killpg(p.pid,signal.SIGKILL);break
  time.sleep(.1)
 exitcode=p.wait()
final=disk()
r={'command':cmd,'pins':pins,'initial_disk':initial,'final_disk':final,'exit_code':exitcode,'stop_reason':stop,'elapsed_seconds':time.monotonic()-start,'peak_sampled_group_rss_bytes':peak,'peak_sampled_group_cpu_seconds':maxcpu,'disk_samples':samples,'limits':{'wall_seconds':600,'group_cpu_seconds':600,'group_rss_bytes':8*G,'per_process_address_space_bytes':8*G,'target_bytes':8*G,'free_floor_bytes':10*G,'disk_sample_interval_seconds':5,'memory_cpu_sample_interval_seconds':.1,'cargo_jobs':2,'rayon_threads':2},'qualification':'local numerical proof only; no signature, authority, correspondence, ledger or Preview acceptance'}
(out/'native-run-receipt.json').write_text(json.dumps(r,indent=2)+'\n');print(json.dumps(r));raise SystemExit(exitcode if exitcode else (1 if stop else 0))
