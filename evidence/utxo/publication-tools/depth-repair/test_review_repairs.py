import unittest,tempfile,json,hashlib,gzip,subprocess,sys,os,shutil
from pathlib import Path
C=Path(__file__).resolve().parent
from publication_policy import inspect

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def archive(h,rows,ex=()):
 h.mkdir();objects={};packed=b'';paths=[]
 for root,name,body in rows:
  digest=hashlib.sha256(body).hexdigest()
  if digest not in objects:
   data=gzip.compress(body,mtime=0);objects[digest]={'sha256':digest,'encoding':'gzip','parts':[{'blob':'blobs-00001.bin','offset':len(packed),'length':len(data)}]};packed+=data
  paths.append({'root':root,'path':name,'sha256':digest,'bytes':len(body)})
 for name,rs in [('objects.jsonl',objects.values()),('paths.jsonl',paths),('exclusions.jsonl',ex),('public-log-extractions.jsonl',[])]: (h/name).write_text(''.join(json.dumps(r)+'\n' for r in rs))
 (h/'blobs-00001.bin').write_bytes(packed);return paths
class ReviewRepairs(unittest.TestCase):
 def test_concrete_secrets_and_public_crypto(self):
  with tempfile.TemporaryDirectory() as tmp:
   p=Path(tmp)
   for i,v in enumerate([{'nested':{'private_key':'a'*64}},{'client_secret':'b'*40},{'mnemonic':'abandon '*11+'about'},{'cborHex':'5820'+'f'*64}]):
    f=p/('case'+str(i)+'.json');f.write_text(json.dumps(v));self.assertTrue(inspect(f)['reason'])
   for typ in ['PlutusScriptV3','PaymentVerificationKeyShelley_ed25519']:
    f=p/'public.json';f.write_text(json.dumps({'type':typ,'cborHex':'5820'+'f'*64}));self.assertIsNone(inspect(f)['reason']);self.assertFalse(inspect(f)['provider_capture'])
   f=p/'graph.json';f.write_text(json.dumps({'directed':True,'nodes':[{'id':'n','label':'n','type':'turn.prompt','text':'DO NOT PUBLISH'}]}));self.assertTrue(inspect(f)['provider_capture'])
 def test_actual_alias_skill_gate_tamper_and_orphan(self):
  with tempfile.TemporaryDirectory(dir=C) as tmp:
   b=Path(tmp);r=b/'research';r.mkdir();v=b/'validation';v.mkdir();env={**os.environ,'UTXO_PUBLICATION_RESEARCH':str(r),'UTXO_PUBLICATION_VALIDATION':str(v)}
   def run(tool,h,where=C,*args):return subprocess.run([sys.executable,str(where/tool),*args],env={**env,'UTXO_PUBLICATION_ARCHIVE':str(h)},capture_output=True,text=True)
   body=json.dumps({'type':'module','name':'public','version':'1'}).encode();h=b/'aliases';archive(h,[('research','a.json',body),('research','raw-model.json',body)]);x=run('filter.py',h);self.assertEqual(x.returncode,0,x.stderr);kept=[json.loads(l)['path'] for l in (h/'paths.jsonl').read_text().splitlines()];self.assertEqual(kept,['a.json']);self.assertEqual(run('finalize.py',h).returncode,0)
   p=r/'.claude/skills/private.md';p.parent.mkdir(parents=True);p.write_text('Unapproved private local instructions.\n');h=b/'skill';archive(h,[],[{'root':'research','path':str(p.relative_to(r)),'sha256':sha(p),'bytes':p.stat().st_size,'reason':'private session or runtime metadata'}]);self.assertEqual(run('filter.py',h).returncode,0);self.assertEqual(run('finalize.py',h).returncode,0);self.assertEqual((h/'paths.jsonl').read_text(),'')
   tools=b/'tools';tools.mkdir()
   for n in ['filter.py','finalize.py','publication_policy.py','provenance_gate.py']:shutil.copyfile(C/n,tools/n)
   h=b/'tamper';archive(h,[('research','public.txt',b'PUBLIC')]);self.assertEqual(run('filter.py',h,tools).returncode,0)
   with (tools/'provenance_gate.py').open('a') as f:f.write('\n# tampered gate bytes\n')
   self.assertNotEqual(run('finalize.py',h,tools).returncode,0)
   h=b/'orphan';rows=archive(h,[('research','excluded.bin',b'EXCLUDED ORIGINAL')]);(h/'paths.jsonl').write_text('');shutil.copyfile(C/'reconstruct.py',h/'reconstruct.py');x=subprocess.run([sys.executable,str(h/'reconstruct.py'),'--sha256',rows[0]['sha256'],'--output',str(b/'leak')],capture_output=True);self.assertNotEqual(x.returncode,0);self.assertFalse((b/'leak').exists())
 def test_actual_selector_anchor_path_and_sidecars(self):
  with tempfile.TemporaryDirectory(dir=C) as tmp:
   b=Path(tmp);r=b/'research';r.mkdir();v=b/'validation';v.mkdir();repo=r/'fixture';repo.mkdir()
   bodies={'public.hs':'{-# OPTIONS --safe #-}\nmodule Public where\n','private-events.hs':'{- HEADER -}\nDO NOT PUBLISH\n','embedded.hs':'{- HEADER -}\n'+json.dumps({'type':'turn.prompt','content':'DO NOT PUBLISH'}),'changed.hs':'{-# OPTIONS --safe #-}\nmodule Changed where\n'}
   for n,body in bodies.items():(repo/n).write_text(body)
   def git(*args):return subprocess.check_output(['git','-C',str(repo),*args],text=True,stderr=subprocess.DEVNULL).strip()
   git('init');git('remote','add','origin','https://example.invalid/public.git');git('add','.');git('-c','user.name=Fixture','-c','user.email=unit@example.invalid','commit','--no-gpg-sign','-m','fixture');commit=git('rev-parse','HEAD');proof=b/'proof.json';proof.write_text(json.dumps([{'root':'research','path':'fixture','commit':commit,'remote':'https://example.invalid/public.git'}]));docproof=b/'docs.json';docproof.write_text('[]');ex=[]
   for n in bodies:
    p=repo/n;ex.append({'root':'research','path':'fixture/'+n,'sha256':sha(p),'bytes':p.stat().st_size,'reason':'raw provider capture identified from exact archived content; strict extraction only'})
   ex.append({'root':'research','path':'fixture/missing.hs','sha256':'a'*64,'bytes':10,'reason':ex[0]['reason']});(repo/'changed.hs').write_text('changed\n');h=b/'capture';archive(h,[],ex);out=b/'selected';args=['--archive',str(h),'--proof',str(proof),'--document-proof',str(docproof),'--research',str(r),'--validation',str(v),'--out',str(out)]
   x=subprocess.run([sys.executable,str(C/'select_restoration.py'),*args],capture_output=True,text=True);self.assertEqual(x.returncode,0,x.stderr);profile=b/'selected-profile.json';d=json.loads(profile.read_text());self.assertEqual([e['original_path'] for e in d['entries']],['fixture/public.hs'])
   env={**os.environ,'PYTHONPATH':str(C),'UTXO_PUBLIC_SOURCE_PROFILE':str(profile)};f=out/'research/fixture/public.hs';code='import json;from publication_policy import inspect;print(json.dumps(inspect('+repr(str(f))+')))'
   self.assertNotEqual(subprocess.run([sys.executable,'-c',code],env=env,capture_output=True).returncode,0)
   env['UTXO_PUBLIC_SOURCE_PROFILE_SHA256']=sha(profile);x=subprocess.run([sys.executable,'-c',code],env=env,capture_output=True,text=True);self.assertFalse(json.loads(x.stdout)['provider_capture'])
   alias=out/'research/unapproved.hs';shutil.copyfile(f,alias);code='import json;from publication_policy import inspect;print(json.dumps(inspect('+repr(str(alias))+')))';x=subprocess.run([sys.executable,'-c',code],env=env,capture_output=True,text=True);self.assertTrue(json.loads(x.stdout)['provider_capture'])
   second=b/'second';target=b/'outside';target.write_text('KEEP');(b/'second-profile.json').symlink_to(target);args[-1]=str(second);x=subprocess.run([sys.executable,str(C/'select_restoration.py'),*args],capture_output=True);self.assertNotEqual(x.returncode,0);self.assertEqual(target.read_text(),'KEEP');self.assertFalse(second.exists())
if __name__=='__main__':unittest.main(verbosity=2)
