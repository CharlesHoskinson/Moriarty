"""Select exact captured public-source false positives, without modifying originals."""
import argparse,json,hashlib,shutil,subprocess
from pathlib import Path
from publication_policy import inspect,confined
from provenance_gate import verify
FORMATS={'.hs','.hs-boot','.lhs','.agda','.toml','.md','.nix','.rtf','.dhall','.jq','.awk','.sh','.py','.js','.mjs','.ts','.rs','.lean','.html'}
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def select(archive,proof,docproof,roots,out):
 out=confined(out,out.parent,roots.values())
 profile=confined(out.parent/(out.name+'-profile.json'),out.parent,roots.values());disposition_file=confined(out.parent/(out.name+'-dispositions.jsonl'),out.parent,roots.values())
 if any(p.exists() or p.is_symlink() for p in [out,profile,disposition_file]):raise ValueError('all restoration outputs must be new and nonsymlink')
 out.mkdir();[(out/n).mkdir() for n in roots];repos=json.loads(proof.read_text());docs=json.loads(docproof.read_text());entries=[];dispositions=[]
 for r in map(json.loads,(archive/'exclusions.jsonl').open()):
  if ('provider capture identified' not in r['reason'] and r['reason']!='private session or runtime metadata') or Path(r['path']).suffix.lower() not in FORMATS:continue
  src=roots[r['root']]/r['path'];row={'root':r['root'],'original_path':r['path'],'sha256':r['sha256'],'bytes':r['bytes']}
  if not src.is_file() or src.is_symlink():dispositions.append({**row,'status':'source-unavailable'});continue
  meta=inspect(src)
  parser_candidate=meta['provider_capture'] and meta['parser_only_candidate'] and not meta['reason']
  skill_candidate=meta['skill_source'] and meta['reason']=='exact public provenance required for skill or command source' and not meta['provider_capture']
  if meta['sha256']!=r['sha256'] or not (parser_candidate or skill_candidate) or meta['provider_marker'] or meta['embedded_private_envelope']:dispositions.append({**row,'status':'not-parser-only-or-source-changed'});continue
  matches=[g for g in repos if g.get('root')==r['root'] and r['path'].startswith(g['path']+'/') and g.get('commit') and g.get('remote')]
  entry=None
  if matches:
   g=max(matches,key=lambda x:len(x['path']));entry={**row,'kind':'git-commit-blob','proof_path':str(proof),'proof_sha256':digest(proof),'repository_original_path':g['path'],'repository_path':str(roots[r['root']]/g['path']),'repository_file':r['path'][len(g['path'])+1:],'commit':g['commit'],'remote':g['remote']}
  else:
   ds=[d for d in docs if d['sha256']==r['sha256'] and d['bytes']==r['bytes']]
   if len(ds)==1:
    d=ds[0];entry={**row,'kind':'public-document-manifest','proof_path':str(docproof),'proof_sha256':digest(docproof),'source_path':str(src),'document_manifest_path':d['path'],'url':d['official_url']}
  if entry is None:dispositions.append({**row,'status':'no-approved-public-provenance'});continue
  try:verify(entry)
  except (ValueError,OSError,subprocess.CalledProcessError) as e:dispositions.append({**row,'status':'provenance-refused','error_class':type(e).__name__});continue
  dst=confined(out/r['root']/r['path'],out,roots.values());dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(src,dst)
  if digest(dst)!=r['sha256']:raise ValueError('source changed while copying selected restoration')
  entries.append(entry);dispositions.append({**row,'status':'exact-public-provenance-selected'})
 with profile.open('x') as f:f.write(json.dumps({'schema':'moriarty-public-source-profile/1','publication_roots':{root:str(out/root) for root in roots},'entries':entries},sort_keys=True,indent=2)+'\n')
 with disposition_file.open('x') as f:f.write(''.join(json.dumps(r,sort_keys=True)+'\n' for r in sorted(dispositions,key=lambda r:(r['root'],r['original_path']))))
 print('selected public-source candidates',len(entries),'profile',profile)
def main():
 p=argparse.ArgumentParser();p.add_argument('--archive',type=Path,required=True);p.add_argument('--proof',type=Path,required=True);p.add_argument('--document-proof',type=Path,required=True);p.add_argument('--research',type=Path,required=True);p.add_argument('--validation',type=Path,required=True);p.add_argument('--out',type=Path,required=True);a=p.parse_args();select(a.archive,a.proof,a.document_proof,{'research':a.research,'validation':a.validation},a.out)
if __name__=='__main__':main()
