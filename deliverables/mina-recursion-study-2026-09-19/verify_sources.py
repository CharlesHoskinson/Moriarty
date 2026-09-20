from pathlib import Path
import json,hashlib,subprocess
r=Path(__file__).parent; rows=[];checks=[]
def read(repo,path,pin,sha):
 p=r/'repos'/repo;b=subprocess.check_output(['git','show',pin+':'+path],cwd=p)
 assert hashlib.sha256(b).hexdigest()==sha,(repo,path);rows.append({'repo':repo,'path':path,'pin':pin,'sha256':sha});return b.decode()
d=json.loads((r/'recursion-semantics-evidence.json').read_text())
for e in d['evidence']:
 s=read(e['repo'],e['path'],e['commit'],e['file_sha256']);lines=s.splitlines()
 for x in e['ranges']:
  excerpt='\n'.join(lines[x['start_line']-1:x['end_line']]);assert excerpt.rstrip('\n')==x['excerpt'].rstrip('\n'),e['id'];checks.append(e['id'])
d=json.loads((r/'developer-evidence.json').read_text())
for claim in d['claims']:
 for e in claim['sources']:
  s=read('o1js',e['path'],d['pin']['commit'],e['file_sha256']);a,b=e['lines_inclusive'];excerpt='\n'.join(s.splitlines()[a-1:b]);assert excerpt.rstrip('\n')==e['excerpt'].rstrip('\n'),(claim['id'],e['path']);checks.append(claim['id'])
unique={(x['repo'],x['path']):x for x in rows};(r/'verified-source-files.json').write_text(json.dumps(list(unique.values()),indent=2));result={'source_files':len(unique),'excerpt_checks':len(checks),'all_passed':True,'scope':'exact pinned source hashes and ranges only; no builds/proofs/tests'};(r/'SOURCE-VERIFICATION.json').write_text(json.dumps(result,indent=2));print(result)
