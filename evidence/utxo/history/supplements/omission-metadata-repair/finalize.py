import json,hashlib,gzip,shutil,collections,re,tempfile,os
from pathlib import Path
H=Path(os.environ.get('UTXO_PUBLICATION_ARCHIVE','/home/charl/Moriarty/.worktrees/utxo/evidence/utxo/history'))
from publication_policy import inspect
import publication_policy
att=json.loads((H/'FILTER-ATTESTATION.json').read_text())
if att.get('schema')!='moriarty-utxo-filter-attestation/1' or att.get('exact_archived_bytes_inspected') is not True:raise ValueError('verified archived-byte filter required')
for name,digest in att['manifest_sha256'].items():
 if name not in ['paths.jsonl','exclusions.jsonl','objects.jsonl','public-log-extractions.jsonl'] or hashlib.sha256((H/name).read_bytes()).hexdigest()!=digest:raise ValueError('stale filter manifest attestation')
if set(att['manifest_sha256'])!={'paths.jsonl','exclusions.jsonl','objects.jsonl','public-log-extractions.jsonl'}:raise ValueError('incomplete filter attestation')
if att.get('provenance_gate_sha256')!=publication_policy.gate_sha256():raise ValueError('stale provenance gate implementation attestation')
if att.get('public_source_profile_sha256')!=publication_policy.profile_sha256():raise ValueError('stale public-source provenance profile attestation')
if att['policy_sha256']!=hashlib.sha256(Path(publication_policy.__file__).read_bytes()).hexdigest():raise ValueError('stale filter policy attestation')
if att['filter_sha256']!=hashlib.sha256((Path(__file__).parent/'filter.py').read_bytes()).hexdigest():raise ValueError('stale filter executable attestation')
paths=[json.loads(l) for l in (H/'paths.jsonl').open()];ex=[json.loads(l) for l in (H/'exclusions.jsonl').open()];public=[json.loads(l) for l in (H/'public-log-extractions.jsonl').open()]
included_keys={(r['root'],r['path']) for r in paths}
ex=[r for r in ex if (r['root'],r['path']) not in included_keys]
allowed={'aa090684582dfce1b4de628fc9e80c9620a4a46db219da841f787b04ad729919','777bba90a16bf170dfd4b86cf1e8db33102513c9f94053f3ce8ae0c95cab36b7'}
objects={r['sha256']:r for r in map(json.loads,(H/'objects.jsonl').open())}
# Restore only exact whole-file hashes independently identified as public UNIT source.
keep=[];newcontent={}
for r in ex:
 public_upstream_skill=r['reason']=='private session or runtime metadata' and any(marker in '/'+r['path'] for marker in ['/.claude/skills/','/.claude/commands/','/.codex/skills/']) and Path(r['path']).suffix in ['.md','.py','.ts','.js','.json','.toml','.yaml','.yml','.txt']
 if (r.get('sha256') in allowed and r['path'].endswith('.test.mjs') and r['reason']=='content credential or private-key signature detected'):
  paths.append({k:v for k,v in r.items() if k!='reason'});root=Path(os.environ.get('UTXO_PUBLICATION_VALIDATION','/home/charl/moriarty-eutxo-validation-2026-10-12') if r['root']=='validation' else os.environ.get('UTXO_PUBLICATION_RESEARCH','/home/charl/moriarty-utxo-research-2026-10-12'));newcontent[r['sha256']]=root/r['path']
  check=inspect(root/r['path'],allowed)
  if check['sha256']!=r['sha256'] or check['reason'] or check['provider_capture']:raise ValueError('unsafe or changed separately restored source')
 else:keep.append(r)
ex=keep
want={r['sha256'] for r in paths}|{r['public_sha256'] for r in public if r.get('public_sha256') is not None}
tmp=Path(tempfile.mkdtemp(prefix='utxo-repack-',dir=H.parent));LIMIT=45*1024*1024;pack=None;num=0;size=0;rows=[]
def put(digest,chunks):
 global pack,num,size
 parts=[]
 for chunk in chunks:
  while chunk:
   if pack is None or size==LIMIT:
    if pack:pack.close()
    num+=1;pack=open(tmp/f'blobs-{num:05d}.bin','wb');size=0
   n=min(len(chunk),LIMIT-size);pack.write(chunk[:n]);parts.append({'blob':f'blobs-{num:05d}.bin','offset':size,'length':n});size+=n;chunk=chunk[n:]
 # Consolidate adjacent ranges in the same blob.
 compact=[]
 for part in parts:
  if compact and compact[-1]['blob']==part['blob'] and compact[-1]['offset']+compact[-1]['length']==part['offset']:compact[-1]['length']+=part['length']
  else:compact.append(part)
 rows.append({'sha256':digest,'encoding':'gzip','parts':compact})
def readparts(r):
 for part in r['parts']:
  with (H/part['blob']).open('rb') as f:
   f.seek(part['offset']);left=part['length']
   while left:
    b=f.read(min(left,1024*1024))
    if not b:raise ValueError('truncated object')
    left-=len(b);yield b
for digest in sorted(want):
 with tempfile.TemporaryFile() as normalized:
  with gzip.GzipFile(filename='',fileobj=normalized,mode='wb',mtime=0,compresslevel=1) as gz:
   h=hashlib.sha256()
   if digest in objects:
    with tempfile.TemporaryFile() as old:
     for b in readparts(objects[digest]):old.write(b)
     old.seek(0)
     with gzip.GzipFile(fileobj=old) as src:
      while b:=src.read(1024*1024):h.update(b);gz.write(b)
   else:
    b=newcontent[digest].read_bytes();h.update(b);gz.write(b)
   assert h.hexdigest()==digest,'normalized object SHA mismatch'
  normalized.seek(0)
  def chunks():
   while b:=normalized.read(1024*1024):yield b
  put(digest,chunks())
if pack:pack.close()
# Retain original source directories; delete only our superseded generated blob files.
for p in H.glob('blobs-*.bin'):p.unlink()
for p in tmp.glob('blobs-*.bin'):p.rename(H/p.name)
tmp.rmdir()
for name,rs in [('paths.jsonl',paths),('exclusions.jsonl',ex),('objects.jsonl',rows)]:
 with (H/name).open('w') as f:
  for r in sorted(rs,key=lambda x:(x.get('root',''),x.get('path',''),x.get('sha256',''))):f.write(json.dumps(r,sort_keys=True)+'\n')
c=collections.Counter();s=collections.Counter()
for r in paths:c['included']+=1;s['included']+=r['bytes']
for r in ex:c[r['reason']]+=1;s[r['reason']]+=r.get('bytes',0)
summary={'counts':dict(c),'original_bytes':dict(s),'unique_objects':len(rows),'archive_blobs':num,'compressed_bytes':sum(p.stat().st_size for p in H.glob('blobs-*.bin')),'public_log_extraction_records':len(public),'public_log_extractions':sum(r.get('public_sha256') is not None for r in public),'roots':{'research':'/home/charl/moriarty-utxo-research-2026-10-12','validation':'/home/charl/moriarty-eutxo-validation-2026-10-12'},'unit_source_exception_sha256':sorted(allowed)}
(H/'SUMMARY.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps(summary,indent=2))
links=[]
for r in ex:
 if r['reason']=='symlink; target not followed':
  root=Path(summary['roots'][r['root']]);p=root/r['path']
  if p.is_symlink():
   target=str(p.readlink())
   if not any(part in r['path'].split('/') for part in ['.git','node_modules','.venv','venv','.cache','.foreman']) and not any(x in target.lower() for x in ['skey','secret','credential','wallet']):links.append({**r,'target':target,'target_sha256':hashlib.sha256(target.encode()).hexdigest(),'kind':'public symlink metadata; no target content followed'})
(H/'symlinks.jsonl').write_text(''.join(json.dumps(r,sort_keys=True)+'\n' for r in links))

(H/'FINALIZATION-GATE.json').write_text(json.dumps({'input_filter_attestation_sha256':hashlib.sha256((H/'FILTER-ATTESTATION.json').read_bytes()).hexdigest(),'input_manifest_sha256':att['manifest_sha256'],'policy_sha256':att['policy_sha256'],'final_manifest_sha256':{name:hashlib.sha256((H/name).read_bytes()).hexdigest() for name in att['manifest_sha256']},'normalized_objects':len(rows),'separately_restored_exact_sha_objects':sorted(newcontent),'source_approval':False,'native_approval':False},sort_keys=True,indent=2)+'\n')
