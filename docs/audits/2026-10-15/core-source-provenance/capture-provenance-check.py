"""One-shot capture of one explicitly classified source-static check; no retries."""
import datetime,gzip,hashlib,json,os,subprocess,sys
from pathlib import Path
R=Path('/home/charl/Moriarty/.moriarty-dev/runtime/core-source-provenance-20261015')
W=Path('/home/charl/Moriarty/.worktrees/core-source-provenance-20261015')
TARGET='/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/cargo-target'
checks={
 'normal-cli':['-p','mori','--test','mapped_role_static_io','-j2','source_step_field_correspondence_survives_public_export_and_reopen','--','--exact','--test-threads=1'],
 'security-cli':['-p','mori','--test','mapped_role_static_io','-j2','provenance_security_','--','--test-threads=1'],
 'normal-ir':['-p','mori-ledger','--test','mapped_role_static','-j2','source_step_fields_match_independent_ir_oracle','--','--exact','--test-threads=1'],
 'security-unit':['-p','mori-ledger','--lib','-j2','relation::emission_tests::','--','--test-threads=1']}
name=sys.argv[1];assert name in checks
attempt=sys.argv[2];assert attempt.replace('-','').isalnum()
out=R/'actual'/attempt;out.mkdir(parents=True,exist_ok=False)
def sha(p):
 h=hashlib.sha256()
 with p.open('rb') as f:
  for b in iter(lambda:f.read(1024*1024),b''):h.update(b)
 return h.hexdigest()
manifest=json.loads((R/'command-final-snapshot-files.json').read_text())
for f in manifest:assert sha(W/f['path'])==f['sha256'],f['path']
assert sha(R/'ORACLE-DESIGN.json')=='33e37dc74dd01bfec329b38eee3adeea462cdb4514e24f0116b75a3081cc1c88'
expected='89ae66c4f8911adaedfec690a0965a88baffc3601c5765fab137c37244eff28e'
assert sha(R/'baseline-mori')==expected
assert not os.environ.get('RUSTFLAGS') and not os.environ.get('CARGO_ENCODED_RUSTFLAGS')
env={k:v for k,v in os.environ.items() if not k.startswith('MORI_')}
env['CARGO_TARGET_DIR']=TARGET
if name=='normal-cli':env['MORI_PROVENANCE_BASELINE_BINARY']=str(R/'baseline-mori')
cmd=['flock','-w','120','/tmp/moriarty-language-wave1-build.lock','timeout','--signal=TERM','--kill-after=10s','300s','env','-u','MORI_FEE_POOL_CAPTURE_DIR','cargo','+1.99.0','--offline','--locked','test']+checks[name]
start=datetime.datetime.now(datetime.timezone.utc).isoformat()
(out/'start.json').write_text(json.dumps({'check':name,'argv':cmd,'cwd':str(W),'started_utc':start,'source_files':manifest,'baseline_sha256':expected,'native_authority':0,'source_expectation_sha256':'33e37dc74dd01bfec329b38eee3adeea462cdb4514e24f0116b75a3081cc1c88','environment_override_keys':['CARGO_TARGET_DIR']+(['MORI_PROVENANCE_BASELINE_BINARY'] if name=='normal-cli' else []),'environment_policy':'all other MORI_* removed in child only; effective Rustflags absent; no full environment captured'},indent=2)+'\n')
p=subprocess.run(cmd,cwd=W,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
for label,b in [('stdout',p.stdout),('stderr',p.stderr)]:
 (out/(label+'.gz')).write_bytes(gzip.compress(b,mtime=0))
unchanged=all(sha(W/f['path'])==f['sha256'] for f in manifest)
baseline=sha(R/'baseline-mori')
result={'check':name,'exit_code':p.returncode,'finished_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'source_unchanged':unchanged,'baseline_sha256_after':baseline,'baseline_unchanged':baseline==expected,'stdout_sha256':hashlib.sha256(p.stdout).hexdigest(),'stdout_bytes':len(p.stdout),'stderr_sha256':hashlib.sha256(p.stderr).hexdigest(),'stderr_bytes':len(p.stderr),'native_operations_requested':0,'scope':'exact selected ordinary check; no unexecuted case or downstream acceptance inferred'}
(out/'result.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result))
print(p.stdout.decode(errors='replace')[-12000:])
print(p.stderr.decode(errors='replace')[-12000:])
sys.exit(p.returncode if p.returncode else (0 if unchanged and baseline==expected else 1))
