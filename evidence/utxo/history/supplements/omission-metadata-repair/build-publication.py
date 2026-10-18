#!/usr/bin/env python3
"""Build a new immutable archive generation; no in-place/resume mutation."""
import argparse,gzip,hashlib,json,os,shutil,stat,subprocess,tempfile
from pathlib import Path
from publication_policy import inspect,extraction,confined,redact_remote
LIMIT=45*1024*1024
UNIT={'aa090684582dfce1b4de628fc9e80c9620a4a46db219da841f787b04ad729919','777bba90a16bf170dfd4b86cf1e8db33102513c9f94053f3ce8ae0c95cab36b7'}
def write_rows(p,rows):
 with p.open('w') as f:
  for r in rows:f.write(json.dumps(r,sort_keys=True)+'\n')
def build(roots,out):
 out=confined(out,Path(out).parent,roots.values())
 if out.exists():raise ValueError('output must be a new generation directory')
 out.parent.mkdir(parents=True,exist_ok=True)
 gen=Path(tempfile.mkdtemp(prefix='.utxo-generation-',dir=out.parent));cache=gen/'object-cache';cache.mkdir();paths=[];ex=[];pub=[];dirsmeta=[];provenance=[];objects={}
 def store(snapshot,digest):
  if digest in objects:return
  dest=cache/(digest+'.gz')
  with dest.open('wb') as raw,gzip.GzipFile(filename='',fileobj=raw,mode='wb',mtime=0,compresslevel=1) as gz,snapshot.open('rb') as inp:shutil.copyfileobj(inp,gz,1024*1024)
  objects[digest]=dest
 try:
  for label,root in sorted(roots.items()):
   root=root.resolve();allowed_packet={}
   packet=root/'t1-gpt61-implementation-417';manifest=packet/'output-manifest.json'
   if manifest.is_file():
    declaration=json.loads(manifest.read_text());allowed_packet={r['path']:r for r in declaration['rows']};allowed_packet.update({'output-manifest.json':None,'output-sidecar.json':None})
   def error(e):ex.append({'root':label,'path':str(Path(e.filename).relative_to(root)),'reason':'walk-error','error_class':type(e).__name__})
   for directory,ds,fs in os.walk(root,followlinks=False,onerror=error):
    d=Path(directory);ds.sort();fs.sort()
    if not ds and not fs:dirsmeta.append({'root':label,'path':str(d.relative_to(root)),'kind':'empty-directory'})
    for n in list(ds):
     p=d/n
     if p.is_symlink():dirsmeta.append({'root':label,'path':str(p.relative_to(root)),'kind':'directory-symlink-not-followed'});ds.remove(n)
    if '.git' in ds:
     try:
      commit=subprocess.check_output(['git','-C',str(d),'rev-parse','HEAD'],text=True,stderr=subprocess.DEVNULL).strip();remote=subprocess.check_output(['git','-C',str(d),'remote','get-url','origin'],text=True,stderr=subprocess.DEVNULL).strip();r={'root':label,'path':str(d.relative_to(root)),'commit':commit,**redact_remote(remote)}
      try:r['default_branch_observed']=subprocess.check_output(['git','-C',str(d),'symbolic-ref','--quiet','--short','refs/remotes/origin/HEAD'],text=True,stderr=subprocess.DEVNULL).strip()
      except subprocess.CalledProcessError:r['default_branch_observed']=None
      provenance.append(r)
     except Exception as e:provenance.append({'root':label,'path':str(d.relative_to(root)),'status':'provenance-unavailable','error_class':type(e).__name__})
    for n in fs:
     p=d/n;rel=str(p.relative_to(root));row={'root':label,'path':rel}
     try:st=p.lstat()
     except OSError as e:ex.append({**row,'reason':'lstat-error','error_class':type(e).__name__});continue
     if stat.S_ISLNK(st.st_mode):ex.append({**row,'reason':'file-symlink-not-followed'});continue
     if not stat.S_ISREG(st.st_mode):ex.append({**row,'reason':'non-regular-file','mode':stat.S_IFMT(st.st_mode)});continue
     if rel.startswith('t1-gpt61-implementation-417/') and rel.split('/',1)[1] not in allowed_packet:
      meta=inspect(p);ex.append({**row,'bytes':meta['bytes'],'sha256':meta['sha256'],'reason':'outside immutable declared public output manifest'});continue
     with tempfile.NamedTemporaryFile(dir=gen,delete=False) as f:snapshot=Path(f.name)
     try:
      with p.open('rb') as src,snapshot.open('wb') as dst:shutil.copyfileobj(src,dst,1024*1024)
      after=p.stat()
      if (st.st_size,st.st_mtime_ns,st.st_ino)!=(after.st_size,after.st_mtime_ns,after.st_ino):ex.append({**row,'reason':'source-mutated-during-capture'});continue
      meta=inspect(snapshot,UNIT,class_path=p);row.update(bytes=meta['bytes'],sha256=meta['sha256'])
      if meta['reason']:ex.append({**row,'reason':meta['reason']});continue
      if meta['provider_capture']:
       body,details=extraction(snapshot);entry={**row,**details};ex.append({**row,'reason':'raw provider capture; strict public extraction only'})
       if body:
        digest=hashlib.sha256(body).hexdigest();snapshot.write_bytes(body);store(snapshot,digest);entry.update(public_sha256=digest,public_bytes=len(body))
       pub.append(entry);continue
      if rel.startswith('t1-gpt61-implementation-417/'):
       expected=allowed_packet[rel.split('/',1)[1]]
       if expected and (expected['sha256']!=row['sha256'] or expected['bytes']!=row['bytes']):raise ValueError('frozen public output mismatch')
      store(snapshot,row['sha256']);paths.append(row)
     except OSError as e:ex.append({**row,'reason':'read-error','error_class':type(e).__name__})
     finally:snapshot.unlink(missing_ok=True)
  packed=[];pack=None;num=0;size=0
  for digest,src in sorted(objects.items()):
   parts=[]
   with src.open('rb') as f:
    while b:=f.read(1024*1024):
     while b:
      if pack is None or size==LIMIT:
       if pack:pack.close()
       num+=1;pack=open(gen/f'blobs-{num:05d}.bin','wb');size=0
      n=min(len(b),LIMIT-size);part={'blob':f'blobs-{num:05d}.bin','offset':size,'length':n};pack.write(b[:n]);size+=n;b=b[n:]
      if parts and parts[-1]['blob']==part['blob']:parts[-1]['length']+=n
      else:parts.append(part)
   packed.append({'sha256':digest,'encoding':'gzip','parts':parts})
  if pack:pack.close()
  shutil.rmtree(cache)
  for name,rs in [('paths.jsonl',paths),('exclusions.jsonl',ex),('public-log-extractions.jsonl',pub),('directory-observations.jsonl',dirsmeta)]:write_rows(gen/name,sorted(rs,key=lambda r:(r['root'],r['path'])))
  write_rows(gen/'objects.jsonl',packed);(gen/'repository-provenance.json').write_text(json.dumps(provenance,sort_keys=True,indent=2)+'\n')
  gen.rename(out);gen=None
 finally:
  if gen is not None:shutil.rmtree(gen)
def main():
 p=argparse.ArgumentParser();p.add_argument('--research',type=Path,required=True);p.add_argument('--validation',type=Path,required=True);p.add_argument('--out',type=Path,required=True);a=p.parse_args();build({'research':a.research,'validation':a.validation},a.out)
if __name__=='__main__':main()
