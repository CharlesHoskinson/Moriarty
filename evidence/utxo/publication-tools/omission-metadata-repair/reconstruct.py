#!/usr/bin/env python3
"""Validate and reconstruct public history objects without trusting manifest paths."""
import argparse,gzip,hashlib,json,os,re,tempfile
from pathlib import Path
BASE=Path(__file__).resolve().parent
SHA=re.compile(r'[0-9a-f]{64}');BLOB=re.compile(r'blobs-[0-9]{5}\.bin')
def rows(name):
 p=BASE/name
 if p.is_symlink():raise ValueError('manifest symlink refused')
 with p.open() as f:
  for line in f:
   r=json.loads(line)
   if not isinstance(r,dict):raise ValueError('manifest row must be an object')
   yield r
def safe_blob(name):
 if not isinstance(name,str) or not BLOB.fullmatch(name):raise ValueError('invalid blob basename')
 p=BASE/name
 if p.is_symlink() or p.resolve().parent!=BASE.resolve() or not p.is_file():raise ValueError('blob outside archive, missing or symlinked')
 return p
def integer(x):
 if type(x) is not int or x<0:raise ValueError('range/size must be nonnegative integer')
 return x
def load():
 objects={};ranges={}
 for r in rows('objects.jsonl'):
  digest=r.get('sha256')
  if not isinstance(digest,str) or not SHA.fullmatch(digest) or digest in objects:raise ValueError('invalid or duplicate object digest')
  if r.get('encoding')!='gzip' or not isinstance(r.get('parts'),list) or not r['parts']:raise ValueError('invalid object encoding/parts')
  for part in r['parts']:
   p=safe_blob(part.get('blob'));off=integer(part.get('offset'));n=integer(part.get('length'))
   if n==0 or off+n>p.stat().st_size:raise ValueError('range empty or exceeds blob')
   ranges.setdefault(p.name,[]).append((off,off+n))
  objects[digest]=r
 paths={}
 for r in rows('paths.jsonl'):
  root=r.get('root');name=r.get('path')
  if root not in ['research','validation'] or not isinstance(name,str) or not name or Path(name).is_absolute() or '..' in Path(name).parts:raise ValueError('invalid original path')
  key=(root,name)
  if key in paths:raise ValueError('duplicate original path')
  if r.get('sha256') not in objects:raise ValueError('path has missing object')
  integer(r.get('bytes'));paths[key]=r
 extracts=[];extraction_keys=set()
 for r in rows('public-log-extractions.jsonl'):
  key=(r.get('root'),r.get('path'))
  if key in extraction_keys:raise ValueError('duplicate public extraction source')
  extraction_keys.add(key)
  if key[0] not in ['research','validation'] or not isinstance(key[1],str) or Path(key[1]).is_absolute() or '..' in Path(key[1]).parts:raise ValueError('invalid extraction source path')
  if 'sha256' in r and r['sha256'] is None:
   # Explicitly unknown source digest: an auditable uninventoried omission that can never carry a public payload.
   if r.get('status')!='source-uninventoried' or not isinstance(r.get('reason'),str) or not r['reason'] or 'public_sha256' not in r or r['public_sha256'] is not None or 'public_bytes' in r or key in paths:raise ValueError('invalid unknown-digest extraction omission')
   continue
  if not isinstance(r.get('sha256'),str) or not SHA.fullmatch(r['sha256']):raise ValueError('invalid extraction source digest')
  if r.get('public_sha256') is not None:
   if r['public_sha256'] not in objects:raise ValueError('extraction has missing object')
   integer(r.get('public_bytes'));extracts.append(r)
 if (BASE/'exclusions.jsonl').exists():
  excluded=set()
  for r in rows('exclusions.jsonl'):
   key=(r.get('root'),r.get('path'))
   if key in excluded or key in paths:raise ValueError('duplicate/conflicting exclusion path')
   excluded.add(key)
 return objects,paths,extracts,ranges
def decode(r,out=None):
 h=hashlib.sha256();n=0
 with tempfile.TemporaryFile() as packed:
  for part in r['parts']:
   with safe_blob(part['blob']).open('rb') as f:
    f.seek(part['offset']);left=part['length']
    while left:
     b=f.read(min(left,1024*1024))
     if not b:raise ValueError('truncated blob')
     packed.write(b);left-=len(b)
  packed.seek(0)
  with gzip.GzipFile(fileobj=packed) as gz:
   while b:=gz.read(1024*1024):
    h.update(b);n+=len(b)
    if out:out.write(b)
 if h.hexdigest()!=r['sha256']:raise ValueError('object SHA mismatch')
 return n
def verify_all(objects,paths,extracts,ranges):
 inventory={p.name for p in BASE.glob('blobs-*.bin')}
 if inventory!=set(ranges):raise ValueError('orphan/missing blob inventory')
 for name,rs in ranges.items():
  off=0
  for start,end in sorted(rs):
   if start!=off:raise ValueError('blob overlap, gap or orphan bytes')
   off=end
  if off!=safe_blob(name).stat().st_size:raise ValueError('trailing blob bytes')
 referenced={r['sha256'] for r in paths.values()}|{r['public_sha256'] for r in extracts}
 if referenced!=set(objects):raise ValueError('orphan object')
 sizes={digest:decode(r) for digest,r in objects.items()}
 for r in paths.values():
  if sizes[r['sha256']]!=r['bytes']:raise ValueError('original path byte-count mismatch')
 for r in extracts:
  if sizes[r['public_sha256']]!=r['public_bytes']:raise ValueError('public extraction byte-count mismatch')
 return sizes
def safe_output(path):
 p=Path(os.path.abspath(path))
 # Refuse every existing symlink component before creating parent directories.
 for c in [p,*p.parents]:
  if c.is_symlink():raise ValueError('output symlink or symlink parent refused')
 p.parent.mkdir(parents=True,exist_ok=True)
 for c in [p,*p.parents]:
  if c.is_symlink():raise ValueError('output symlink or symlink parent refused')
 return p
def atomic_output(r,destination,expected=None):
 p=safe_output(destination);tmp=None
 try:
  with tempfile.NamedTemporaryFile(dir=p.parent,prefix='.utxo-reconstruct-',delete=False) as f:
   tmp=Path(f.name);n=decode(r,f);f.flush();os.fsync(f.fileno())
  if expected is not None and n!=expected:raise ValueError('original path byte-count mismatch')
  safe_output(p);os.replace(tmp,p);tmp=None
  return n
 finally:
  if tmp is not None:tmp.unlink(missing_ok=True)
def main():
 p=argparse.ArgumentParser();p.add_argument('--root',choices=['research','validation']);p.add_argument('--path');p.add_argument('--sha256');p.add_argument('--output',type=Path);p.add_argument('--verify-all',action='store_true');a=p.parse_args()
 objects,paths,extracts,ranges=load()
 if a.verify_all:
  sizes=verify_all(objects,paths,extracts,ranges);print('verified',len(sizes),'unique objects and',len(paths),'original paths');return
 if not a.output:p.error('--output required')
 if a.sha256:
  if a.sha256 not in objects:p.error('unknown object digest')
  # Hash mode must not expose orphan originals in an intermediate generation.
  sizes=verify_all(objects,paths,extracts,ranges)
  digest=a.sha256;expected=sizes[digest]
 else:
  if not a.root or not a.path:p.error('--root and --path required')
  r=paths.get((a.root,a.path))
  if r is None:p.error('original path absent or excluded')
  digest=r['sha256'];expected=r['bytes']
 n=atomic_output(objects[digest],a.output,expected);print(digest,n,str(a.output))
if __name__=='__main__':main()
