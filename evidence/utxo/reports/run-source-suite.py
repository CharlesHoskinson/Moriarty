from pathlib import Path
import subprocess,json,datetime,hashlib
root=Path(__file__).parent
rows=[]
for folder in sorted((root/'designers').iterdir()):
 if not folder.is_dir():continue
 script=folder/'verify.mjs'
 if not script.exists():raise RuntimeError('Missing executable independent verifier: '+str(script))
 result=subprocess.run(['node',str(script)],cwd=folder,text=True,capture_output=True)
 (root/('suite-'+folder.name+'.log')).write_text(result.stdout+result.stderr)
 rows.append({'designer':folder.name,'command':'node '+str(script),'exit_code':result.returncode,'verifier_sha256':hashlib.sha256(script.read_bytes()).hexdigest(),'log':str(root/('suite-'+folder.name+'.log'))})
 if result.returncode:raise RuntimeError('Source verifier failed: '+folder.name)
receipt={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'tier':'actual Moriarty source APIs and atomic Simulation; no deployment-owned required-claim backend','pattern_count':len(json.loads((root/'PATTERN-INVENTORY.json').read_text())),'verifiers':rows,'status':'all independent source verifiers pass','native_tier':'not implied','private_proof_tier':'unsupported, rejecting probes executed'}
(root/'SOURCE-SUITE-RESULTS.json').write_text(json.dumps(receipt,indent=2)+'\n');print(json.dumps(receipt,indent=2))
