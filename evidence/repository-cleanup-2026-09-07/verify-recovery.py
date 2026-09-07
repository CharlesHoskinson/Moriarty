from pathlib import Path
import subprocess,json,hashlib,re,datetime
r=Path('/home/charl/Moriarty');out=r/'evidence/repository-cleanup-2026-09-07';m=json.loads((out/'archive-manifest.json').read_text());backup=Path('/home/charl/backups/moriarty/2026-09-07/before-cleanup.bundle')
def run(a):return subprocess.run(a,cwd=r,capture_output=True)
v=run(['git','bundle','verify',str(backup)]);(out/'bundle-verify.txt').write_bytes(v.stdout+v.stderr)
ancestry=[{'ref':x['ref'],'commit':x['commit'],'exit_code':run(['git','merge-base','--is-ancestor',x['commit'],m['recovery_commit']]).returncode} for x in m['branches_before'] if not x['ref'].endswith('/HEAD')]
check={'bundle_sha256':hashlib.file_digest(backup.open('rb'),'sha256').hexdigest(),'bundle_bytes':backup.stat().st_size,'verify_exit_code':v.returncode,'ancestry':ancestry,'manifest_sha256':hashlib.sha256((out/'archive-manifest.json').read_bytes()).hexdigest()}
(out/'recovery-verification.json').write_text(json.dumps(check,indent=2)+'\n')
# No matched bytes are printed or persisted. A bounded pattern check, not a guarantee of absence.
patterns={
 'private-key':rb'-----BEGIN (?:RSA |EC |OPENSSH |DSA )?PRIVATE KEY-----',
 'github-token':rb'\b(?:gh[pousr]_[A-Za-z0-9]{30,255}|github_pat_[A-Za-z0-9_]{60,255})\b',
 'aws-access-id':rb'\b(?:AKIA|ASIA)[A-Z0-9]{16}\b',
 'provider-key':rb'\bsk-(?:ant-api\d+-|proj-)[A-Za-z0-9_-]{30,255}',
}
compiled={k:re.compile(v) for k,v in patterns.items()};objects=run(['git','rev-list','--objects','HEAD']).stdout.decode().splitlines();mapping={line.split(' ',1)[0]:line.partition(' ')[2] for line in objects};proc=subprocess.Popen(['git','cat-file','--batch'],cwd=r,stdin=subprocess.PIPE,stdout=subprocess.PIPE);hits=[];scanned=0;total=0
for oid,path in mapping.items():
 proc.stdin.write((oid+'\n').encode());proc.stdin.flush();header=proc.stdout.readline().decode().split();size=int(header[2]);data=proc.stdout.read(size);proc.stdout.read(1)
 if header[1]=='blob':
  scanned+=1;total+=size
  for name,pat in compiled.items():
   if pat.search(data):hits.append({'object':oid,'path':path,'pattern':name})
proc.stdin.close();proc.wait()
working=[]
for path in run(['git','ls-files','--modified','--others','--exclude-standard']).stdout.decode().splitlines():
 p=r/path
 if not p.is_file():continue
 data=p.read_bytes()
 for name,pat in compiled.items():
  if pat.search(data):working.append({'path':path,'pattern':name})
(out/'publication-pattern-scan.json').write_text(json.dumps({'observed_at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'scope':'All blobs reachable from recovery HEAD, plus extant modified/untracked files at scan time','recovery_commit':m['recovery_commit'],'patterns':{k:v.decode() for k,v in patterns.items()},'blobs_scanned':scanned,'bytes_scanned':total,'findings':hits,'working_tree_findings':working,'limits':'Known credential patterns only; no entropy, seed-phrase or semantic secret detection. Existing origin history is already public. No clean-security guarantee.'},indent=2)+'\n')
print(json.dumps({'bundle_verified':v.returncode==0,'ancestry_passed':all(x['exit_code']==0 for x in ancestry),'blobs_scanned':scanned,'findings':len(hits),'working_tree_findings':len(working)}))
