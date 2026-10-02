# Source-only bounded child helper; supervisor owns process-group/wall/CPU/RSS/disk limits.
import sys,json,hashlib,subprocess,os
from pathlib import Path
base=Path('/home/charl/research/moriarty-signed-intent-2026-10-01')
assert len(sys.argv)==6,'ELF ELF_SHA CONFIG CONFIG_SHA NEW_OUTPUT required'
elf=Path(sys.argv[1]);elfpin=sys.argv[2];config=Path(sys.argv[3]);configpin=sys.argv[4];out=Path(sys.argv[5])
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def exclusive(path,body):
 with Path(path).open('xb') as f:f.write(body);f.flush();os.fsync(f.fileno())
def encode(v):return (json.dumps(v,indent=2)+'\n').encode()
assert sha(elf)==elfpin and sha(config)==configpin
cfg=json.loads(config.read_bytes());assert set(cfg)=={'vk','srs','proof','statement','transaction','genesis','expected_contract','funding_history','funding_claim','fixture'}
for n in set(cfg)-{'fixture'}:assert set(cfg[n])=={'path','sha256'} and sha(cfg[n]['path'])==cfg[n]['sha256']
spec_path=base/'NATIVE-FINANCIAL-V10-HISTORY-CASES.json'
assert sha(spec_path)=='103fa0e30387d64464e8672bc35eba7ba889a93529566bea2e1bf0069f12fa47'
spec=json.loads(spec_path.read_bytes());assert cfg['funding_history']['sha256']==spec['base_history_sha256'] and cfg['funding_claim']['sha256']==spec['base_claim_sha256']
assert not out.exists()
# Each CLI invocation keeps the inherited supervisor process group/network namespace/rlimits.
def run(mode,c,cpath,directory,logpath):
 exclusive(cpath,encode(c));pin=sha(cpath)
 command=[str(elf),mode,str(cpath),pin,str(directory)]
 with logpath.open('xb') as log:
  result=subprocess.run(command,stdout=log,stderr=subprocess.STDOUT)
  log.flush();os.fsync(log.fileno())
 assert sha(cpath)==pin and sha(elf)==elfpin
 raw=logpath.read_text();print(raw,flush=True)
 return result.returncode,raw,{'command':command,'config_sha256':pin,'exit_code':result.returncode,'log_sha256':sha(logpath)}
# Good first uses the exact root-created config; native writes fresh output root.
first_log=base/'native-financial-v10-r4-verify-good-first.log';assert not first_log.exists()
with first_log.open('xb') as log:
 first=subprocess.run([str(elf),'verify',str(config),configpin,str(out)],stdout=log,stderr=subprocess.STDOUT);log.flush();os.fsync(log.fileno())
print(first_log.read_text(),flush=True);assert first.returncode==0 and out.is_dir(),'actual independent good control failed'
faults=out/'fault-controls';faults.mkdir();records=[]
# Native alternative producer uses public fixed fixture only; no external signer or keys.
prepare=json.loads((base/'native-ledger-v5-keyless-prepare-config.json').read_bytes());prepare['fixture']=dict(cfg['fixture']);prepare['fixture']['night_value']+=1;prepare['fixture']['night_creation_seconds']+=1
alt=faults/'alternate-funding'
code,raw,rec=run('funding-preflight',prepare,faults/'alternate-config.json',alt,faults/'alternate.log');assert code==0
assert {x.name for x in alt.iterdir()}=={'funded-genesis.tagged','funding-history.tagged','funding-claim.tagged','funding-history.json','funding-preflight.json'}
p=json.loads((alt/'funding-preflight.json').read_bytes());assert p['result']=='Success' and p['native_default_well_formed'] is True and p['native_claim_applied'] is True and p['supply_invariants'] is True and p['failed_originals_unchanged'] is True and p['pristine_good_again_equal'] is True and p['proof_produced'] is False and p['financial_ledger_accepted'] is False
records.append({'case':'canonical-native-alternate-producer','classification':'local native public-fixture producer only',**rec})
controls=dict(spec['cases'])
for name,field,file in [('canonical-alternate-history','funding_history','funding-history.tagged'),('canonical-alternate-claim','funding_claim','funding-claim.tagged')]:
 path=alt/file;assert sha(path)!=cfg[field]['sha256']
 controls[name]={'path':str(path),'sha256':sha(path),'bytes':path.stat().st_size,'field':field,'classification':'native deterministic history/claim reconstruction refusal','expected_error':'supervisor-pinned funding predecessor history differs from independent native reconstruction'}
for name,a in controls.items():
 assert sha(a['path'])==a['sha256'] and Path(a['path']).stat().st_size==a['bytes']
 mutated=dict(cfg);mutated[a['field']]={'path':a['path'],'sha256':a['sha256']}
 directory=faults/(name+'-output');code,raw,rec=run('verify',mutated,faults/(name+'-config.json'),directory,faults/(name+'.log'))
 assert code==1 and not directory.exists(),'mutation accepted or crashed outside expected returned error'
 assert 'host artifact identity mismatch' not in raw and 'invalid supervisor SHA256' not in raw
 if a['expected_error'] is not None:assert a['expected_error'] in raw
 else:assert 'Error:' in raw,'truncation must return actual native decoder error'
 records.append({'case':name,'classification':a['classification'],'artifact_sha256':a['sha256'],**rec})
# Repeat complete pristine native verifier after all failures.
good=faults/'good-again';code,raw,rec=run('verify',cfg,faults/'good-again-config.json',good,faults/'good-again.log');assert code==0
assert (good/'independent-ledger-receipt.json').read_bytes()==(out/'independent-ledger-receipt.json').read_bytes()
records.append({'case':'pristine-good-again','classification':'actual complete independent native verifier',**rec})
for n in set(cfg)-{'fixture'}:assert sha(cfg[n]['path'])==cfg[n]['sha256']
exclusive(out/'history-control-results.json',encode({'scope':'actual public native CLI decoder/history controls only; generic history/PCD remains open','controls':records,'all_six_refusals':True,'pristine_good_again':True,'alternative_history_qualification':spec['alternate_history_qualification']}))
print('NATIVE_PUBLIC_HISTORY_CONTROL_SUITE_GOOD_AGAIN_OK',flush=True)
