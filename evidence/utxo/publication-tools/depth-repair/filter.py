"""Reclassify exact archived bytes and rebuild strict public-only extractions."""
import collections,gzip,hashlib,json,tempfile,shutil,os
from pathlib import Path
from publication_policy import inspect,extraction,filename_reason,classification_context
H=Path(os.environ.get('UTXO_PUBLICATION_ARCHIVE','/home/charl/Moriarty/.worktrees/utxo/evidence/utxo/history'));ROOTS={'research':Path(os.environ.get('UTXO_PUBLICATION_RESEARCH','/home/charl/moriarty-utxo-research-2026-10-12')),'validation':Path(os.environ.get('UTXO_PUBLICATION_VALIDATION','/home/charl/moriarty-eutxo-validation-2026-10-12'))}
UNIT={'aa090684582dfce1b4de628fc9e80c9620a4a46db219da841f787b04ad729919','777bba90a16bf170dfd4b86cf1e8db33102513c9f94053f3ce8ae0c95cab36b7'}
paths=list(map(json.loads,(H/'paths.jsonl').open()));ex=list(map(json.loads,(H/'exclusions.jsonl').open()));objects={r['sha256']:r for r in map(json.loads,(H/'objects.jsonl').open())};groups=collections.defaultdict(list)
for r in paths:groups[r['sha256']].append(r)
keep=[];public=[];bodies={};raw_seen=set()
def decode(r,dst):
 with tempfile.TemporaryFile() as tmp:
  for part in r['parts']:
   with (H/part['blob']).open('rb') as f:
    f.seek(part['offset']);left=part['length']
    while left:
     b=f.read(min(left,1024*1024));assert b;left-=len(b);tmp.write(b)
  tmp.seek(0)
  with gzip.GzipFile(fileobj=tmp) as src,dst.open('wb') as out:shutil.copyfileobj(src,out,1024*1024)
def extract(row,path):
 body,details=extraction(path);entry={**row,**details};raw_seen.add((row['root'],row['path']))
 if body:
  digest=hashlib.sha256(body).hexdigest();bodies[digest]=body;entry.update(public_sha256=digest,public_bytes=len(body))
 public.append(entry)
for digest,rs in sorted(groups.items()):
 with tempfile.NamedTemporaryFile() as tmp:
  p=Path(tmp.name);decode(objects[digest],p);contexts={}
  for r in rs:
   original=ROOTS[r['root']]/r['path'];ctx=classification_context(original)
   if ctx not in contexts:contexts[ctx]=inspect(p,UNIT,class_path=original)
   meta=contexts[ctx];assert meta['sha256']==digest,'captured byte mismatch';reason=filename_reason(original) or meta['reason'];suffix=original.suffix.lower();provider=meta['provider_capture'] or suffix in ['.jsonl','.ndjson']
   if reason:ex.append({**r,'reason':reason})
   elif provider:
    ex.append({**r,'reason':'raw provider capture identified from exact archived content; strict extraction only'});extract(r,p)
   else:keep.append(r)
for r in list(ex):
 key=(r['root'],r['path']);source=ROOTS[r['root']]/r['path']
 if key in raw_seen or filename_reason(source):continue
 if source.suffix.lower() in ['.jsonl','.ndjson'] or 'provider' in r['reason'] or 'reasoning envelope' in r['reason']:
  if not source.is_file() or source.is_symlink():public.append({**r,'status':'source-unavailable','public_sha256':None});continue
  meta=inspect(source,UNIT)
  if meta['sha256']!=r.get('sha256'):public.append({**r,'status':'source-changed-since-recorded-digest','public_sha256':None});continue
  extract({k:v for k,v in r.items() if k!='reason'},source)
num=max(int(p.stem.split('-')[1]) for p in H.glob('blobs-*.bin'));pack=None;size=0;LIMIT=45*1024*1024
with (H/'objects.jsonl').open('a') as out:
 for digest,body in sorted(bodies.items()):
  if digest in objects:continue
  data=gzip.compress(body,compresslevel=1,mtime=0);parts=[]
  while data:
   if pack is None or size==LIMIT:
    if pack:pack.close()
    num+=1;pack=open(H/f'blobs-{num:05d}.bin','wb');size=0
   n=min(len(data),LIMIT-size);pack.write(data[:n]);parts.append({'blob':f'blobs-{num:05d}.bin','offset':size,'length':n});size+=n;data=data[n:]
  out.write(json.dumps({'sha256':digest,'encoding':'gzip','parts':parts},sort_keys=True)+'\n')
if pack:pack.close()
for name,rs in [('paths.jsonl',keep),('exclusions.jsonl',ex),('public-log-extractions.jsonl',public)]:
 with (H/name).open('w') as out:
  for r in sorted(rs,key=lambda x:(x['root'],x['path'])):out.write(json.dumps(r,sort_keys=True)+'\n')
print('strict-filter paths',len(keep),'exclusions',len(ex),'extraction records',len(public))

import publication_policy
manifest_pins={name:hashlib.sha256((H/name).read_bytes()).hexdigest() for name in ['paths.jsonl','exclusions.jsonl','objects.jsonl','public-log-extractions.jsonl']}
att={'provenance_gate_sha256':publication_policy.gate_sha256(),'public_source_profile_sha256':publication_policy.profile_sha256(),'schema':'moriarty-utxo-filter-attestation/1','manifest_sha256':manifest_pins,'policy_sha256':hashlib.sha256(Path(publication_policy.__file__).read_bytes()).hexdigest(),'filter_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'exact_archived_bytes_inspected':True,'all_suffixes_and_sizes':True,'source_approval':False,'native_approval':False}
(H/'FILTER-ATTESTATION.json').write_text(json.dumps(att,sort_keys=True,indent=2)+'\n')
