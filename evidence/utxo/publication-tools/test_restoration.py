import unittest,tempfile,subprocess,json,hashlib,os,sys,shutil
from pathlib import Path
HERE=Path(__file__).resolve().parent
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
class RestorationTests(unittest.TestCase):
 def test_provenance_bound_actual_chain_and_poison(self):
  with tempfile.TemporaryDirectory(dir=HERE) as tmp:
   b=Path(tmp);repo=b/'repo';repo.mkdir();research=b/'research';research.mkdir();validation=b/'validation';validation.mkdir()
   positives={'public.hs':'{-# LANGUAGE OverloadedStrings #-}\nmodule Public where\nx = 1\n','Cargo.toml':'[package]\nname = "public"\nversion = "1.0.0"\n','README.md':'[Public documentation](https://example.invalid/docs)\n\nPublic overview.\n','public.nix':'{ pkgs, ... }: pkgs.hello\n','public.agda':'{-# OPTIONS --safe #-}\nmodule Public where\n'}
   negatives={'capture.hs':json.dumps({'type':'session.started','content':'DO NOT PUBLISH'}),'capture.weird':'{-# OPTIONS --safe #-}\nnot a public source format\n','nested.md':json.dumps({'nested':{'type':'new.unknown','content':'DO NOT PUBLISH'}}),'embedded.hs':'{-# OPTIONS --safe #-}\n'+json.dumps({'type':'turn.prompt','content':'DO NOT PUBLISH'}),'partial.md':'{"type":"new.unknown","content":"DO NOT PUBLISH"','review-private-events.md':'[Public](https://example.invalid)\nDO NOT PUBLISH','secret.md':'[Public](https://example.invalid)\napi_key='+'z'*40}
   for n,body in {**positives,**negatives}.items():(repo/n).write_text(body);(research/n).write_text(body)
   def git(*args):return subprocess.check_output(['git','-C',str(repo),*args],stderr=subprocess.DEVNULL,text=True).strip()
   git('init');git('remote','add','origin','https://example.invalid/public/source.git');git('add','.');git('-c','user.name=PublicationFixture','-c','user.email=unit@example.invalid','commit','--no-gpg-sign','-m','public source fixture');commit=git('rev-parse','HEAD')
   proof=b/'collection.json';proof.write_text(json.dumps([{'root':'research','path':'fixture','commit':commit,'remote':'https://example.invalid/public/source.git'}]))
   entries=[]
   for n in [*positives,*negatives]:
    p=repo/n;entries.append({'kind':'git-commit-blob','root':'research','original_path':n,'sha256':sha(p),'bytes':p.stat().st_size,'proof_path':str(proof),'proof_sha256':sha(proof),'repository_original_path':'fixture','repository_path':str(repo),'repository_file':n,'commit':commit,'remote':'https://example.invalid/public/source.git'})
   document=b/'official.md';document.write_text('[Official](https://example.invalid/docs)\nPublic documentation.\n');shutil.copyfile(document,research/'official.md');docproof=b/'document-proof.json';docproof.write_text(json.dumps([{'path':'official.md','sha256':sha(document),'bytes':document.stat().st_size,'official_url':'https://example.invalid/docs'}]));entries.append({'kind':'public-document-manifest','root':'research','original_path':'official.md','sha256':sha(document),'bytes':document.stat().st_size,'proof_path':str(docproof),'proof_sha256':sha(docproof),'source_path':str(document),'document_manifest_path':'official.md','url':'https://example.invalid/docs'})
   profile=b/'profile.json';profile.write_text(json.dumps({'schema':'moriarty-public-source-profile/1','publication_roots':{'research':str(research),'validation':str(validation)},'entries':entries}));env={**os.environ,'UTXO_PUBLIC_SOURCE_PROFILE':str(profile),'UTXO_PUBLIC_SOURCE_PROFILE_SHA256':sha(profile),'UTXO_PUBLICATION_ARCHIVE':str(b/'archive'),'UTXO_PUBLICATION_RESEARCH':str(research),'UTXO_PUBLICATION_VALIDATION':str(validation)}
   run=lambda tool,*args:subprocess.run([sys.executable,str(HERE/tool),*args],env=env,capture_output=True,text=True)
   built=run('build-publication.py','--research',str(research),'--validation',str(validation),'--out',str(b/'archive'));self.assertEqual(built.returncode,0,built.stderr)
   filtered=run('filter.py');self.assertEqual(filtered.returncode,0,filtered.stderr);att=json.loads((b/'archive/FILTER-ATTESTATION.json').read_text());self.assertEqual(att['public_source_profile_sha256'],sha(profile))
   final=run('finalize.py');self.assertEqual(final.returncode,0,final.stderr);shutil.copyfile(HERE/'reconstruct.py',b/'archive/reconstruct.py')
   verified=subprocess.run([sys.executable,str(b/'archive/reconstruct.py'),'--verify-all'],capture_output=True,text=True);self.assertEqual(verified.returncode,0,verified.stderr)
   rows=[json.loads(l) for l in (b/'archive/paths.jsonl').read_text().splitlines()];self.assertEqual({r['path'] for r in rows},set(positives)|{'official.md'})
   for row in rows:self.assertEqual(row['sha256'],sha(research/row['path']))
   profile.write_text(profile.read_text()+'\n');self.assertNotEqual(run('finalize.py').returncode,0,'changed provenance profile must refuse finalization')
   print('actual source restoration chain:',verified.stdout.strip(),'public formats retained; 7 poison/key/provenance controls withheld')
 def test_missing_and_forged_provenance_fail_closed(self):
  with tempfile.TemporaryDirectory() as tmp:
   b=Path(tmp);source=b/'public.hs';source.write_text('{-# OPTIONS --safe #-}\nmodule Public where\n')
   code='import json;from publication_policy import inspect;print(json.dumps(inspect('+repr(str(source))+')))'
   base={**os.environ,'PYTHONPATH':str(HERE)};base.pop('UTXO_PUBLIC_SOURCE_PROFILE',None)
   raw=subprocess.run([sys.executable,'-c',code],env=base,capture_output=True,text=True);self.assertTrue(json.loads(raw.stdout)['provider_capture'])
   proof=b/'forged.json';proof.write_text('[]');profile=b/'profile.json';profile.write_text(json.dumps({'schema':'moriarty-public-source-profile/1','publication_roots':{'research':str(b)},'entries':[{'kind':'public-document-manifest','root':'research','original_path':'public.hs','sha256':sha(source),'bytes':source.stat().st_size,'proof_path':str(proof),'proof_sha256':sha(proof),'source_path':str(source),'document_manifest_path':'public.hs','url':'https://example.invalid/docs'}]}))
   forged=subprocess.run([sys.executable,'-c',code],env={**base,'UTXO_PUBLIC_SOURCE_PROFILE':str(profile),'UTXO_PUBLIC_SOURCE_PROFILE_SHA256':sha(profile)},capture_output=True,text=True);self.assertEqual(forged.returncode,0);self.assertTrue(json.loads(forged.stdout)['provider_capture'])
   print('missing/forged public-source provenance refused')
 def test_actual_git_blob_remote_and_manifest_mismatches(self):
  from provenance_gate import verify
  with tempfile.TemporaryDirectory() as tmp:
   b=Path(tmp);repo=b/'repo';repo.mkdir();source=repo/'public.hs';source.write_text('{-# OPTIONS --safe #-}\nmodule Public where\n')
   def git(*args):return subprocess.check_output(['git','-C',str(repo),*args],stderr=subprocess.DEVNULL,text=True).strip()
   git('init');git('remote','add','origin','https://example.invalid/public/source.git');git('add','.');git('-c','user.name=PublicationFixture','-c','user.email=unit@example.invalid','commit','--no-gpg-sign','-m','public fixture');commit=git('rev-parse','HEAD')
   proof=b/'proof.json';proof.write_text(json.dumps([{'root':'research','path':'fixture','commit':commit,'remote':'https://example.invalid/public/source.git'}]));entry={'kind':'git-commit-blob','root':'research','original_path':'public.hs','sha256':sha(source),'bytes':source.stat().st_size,'proof_path':str(proof),'proof_sha256':sha(proof),'repository_original_path':'fixture','repository_path':str(repo),'repository_file':'public.hs','commit':commit,'remote':'https://example.invalid/public/source.git'}
   self.assertTrue(verify(entry));source.write_text(source.read_text()+'changed = True\n')
   with self.assertRaises(ValueError):verify({**entry,'sha256':sha(source),'bytes':source.stat().st_size})
   git('remote','set-url','origin','https://example.invalid/wrong.git')
   with self.assertRaises(ValueError):verify(entry)
   git('remote','set-url','origin',entry['remote']);proof.write_text(proof.read_text()+'\n')
   with self.assertRaises(ValueError):verify(entry)
   print('actual wrong Git blob, changed remote and changed proof manifest refused')
if __name__=='__main__':unittest.main(verbosity=2)
