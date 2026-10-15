import unittest,tempfile,json,gzip,hashlib,importlib.util,sys
from pathlib import Path
from publication_policy import inspect,extraction,redact_remote,confined
from importlib.machinery import SourceFileLoader
HERE=Path(__file__).resolve().parent
RECON=SourceFileLoader('reconstruction',str(HERE/'reconstruct.py')).load_module()
from importlib.machinery import SourceFileLoader
BUILD=SourceFileLoader('safe_builder',str(HERE/('build-publication.py' if (HERE/'build-publication.py').exists() else 'build-safe.py'))).load_module()
class PublicationTests(unittest.TestCase):
 def test_provider_poison_and_strict_allowlist(self):
  with tempfile.TemporaryDirectory() as d:
   p=Path(d)
   for suffix in ['.log','.json','.stdout','.weird']:
    for event in [{'type':'thinking','data':'private'},{'unknown':{'reasoning_content':'private'}},[{'wrapper':{'thoughts':'private'}}],{'type':'new.unknown','data':'private'},{'wrapped':json.dumps({'type':'session.started','content':'private'})}]:
     f=p/('capture'+suffix);f.write_text(json.dumps(event));self.assertTrue(inspect(f)['provider_capture']);self.assertEqual(extraction(f)[0],b'')
   f=p/'large.stdout';f.write_text(json.dumps({'type':'thinking','data':'x'*(21*1024*1024)}));self.assertTrue(inspect(f)['provider_capture']);self.assertEqual(extraction(f)[0],b'')
   f=p/'graph.json';f.write_text(json.dumps({'directed':True,'nodes':[{'reasoning':'public explanation','source_file':'trace.jsonl'}]}));self.assertFalse(inspect(f)['provider_capture'])
   f.write_text(json.dumps({'directed':True,'nodes':[{'text':json.dumps({'payload_type':'run.thought.delta','payload':{'text':'private'}})}]}));self.assertTrue(inspect(f)['provider_capture'])
   f=p/'events.jsonl';events=[{'type':'text','data':'PUBLIC'},{'payload_type':'text','data':'POISON'},{'type':'text','payload_type':'run.thought.delta','data':'POISON'},{'type':'run.output.delta','text':'POISON'},{'payload_type':'run.output.delta','text':'POISON'},{'payload_type':'run.output.delta','payload':{'text':'MUSE'}},{'payload_type':'run.terminal.completed','payload':{'text':'FINAL'}}];f.write_text('\n'.join(map(json.dumps,events)));self.assertEqual(extraction(f)[0],b'PUBLIC\nMUSE\nFINAL')
 def test_secret_classes_remote_and_destination(self):
  with tempfile.TemporaryDirectory() as d:
   p=Path(d)
   for name,body in [('extensionless','5820'+'a'*64),('.env','API_KEY='+'z'*40),('credentials','secret'),('keyblob',json.dumps({'type':'PaymentSigningKeyShelley_ed25519','cborHex':'5820'+'a'*64}))]:
    f=p/name;f.write_text(body);self.assertTrue(inspect(f)['reason'],name)
   for name,body in [('seed.unknown',b'\x01'*32),('extended.unknown',b'\x02'*96),('.env.local',b'API_URL=https://example.com')]:
    f=p/name;f.write_bytes(body);self.assertTrue(inspect(f)['reason'])
   for location in ['downloaded-tools/runtime.tar.xz','.pytest_cache/cache.txt']:
    f=p/location;f.parent.mkdir(parents=True,exist_ok=True);f.write_text('public but runtime/cache');self.assertTrue(inspect(f)['reason'])
   self.assertNotIn('PASSWORD',redact_remote('ssh://user:PASSWORD@example.com/project.git')['remote']);self.assertNotIn('TOKEN',redact_remote('user:TOKEN@example.com:project.git')['remote']);self.assertNotIn('TOKEN',redact_remote('https://example.com/x?token=TOKEN')['remote'])
   alias=p/'alias';alias.symlink_to(p/'actual',target_is_directory=True)
   with self.assertRaises(ValueError):confined(alias/'file',p)
 def manifest(self,p):
  body=b'PUBLIC BYTES';dg=hashlib.sha256(body).hexdigest();b=gzip.compress(body,mtime=0);(p/'blobs-00001.bin').write_bytes(b)
  obj={'sha256':dg,'encoding':'gzip','parts':[{'blob':'blobs-00001.bin','offset':0,'length':len(b)}]};row={'root':'research','path':'public.txt','sha256':dg,'bytes':len(body)}
  (p/'objects.jsonl').write_text(json.dumps(obj)+'\n');(p/'paths.jsonl').write_text(json.dumps(row)+'\n');(p/'public-log-extractions.jsonl').write_text('');return obj,row
 def test_manifest_and_atomic_output_rejections(self):
  with tempfile.TemporaryDirectory() as d:
   p=Path(d);RECON.BASE=p;obj,row=self.manifest(p)
   self.assertEqual(RECON.verify_all(*RECON.load())[row['sha256']],row['bytes'])
   for name in ['/etc/passwd','../outside','blobs-00001.bin/../outside']:
    bad=json.loads(json.dumps(obj));bad['parts'][0]['blob']=name;(p/'objects.jsonl').write_text(json.dumps(bad)+'\n')
    with self.assertRaises(ValueError):RECON.load()
   for key,value in [('offset',-1),('length',True),('length',999999)]:
    bad=json.loads(json.dumps(obj));bad['parts'][0][key]=value;(p/'objects.jsonl').write_text(json.dumps(bad)+'\n')
    with self.assertRaises(ValueError):RECON.load()
   self.manifest(p);(p/'objects.jsonl').write_text((json.dumps(obj)+'\n')*2)
   with self.assertRaises(ValueError):RECON.load()
   self.manifest(p);(p/'paths.jsonl').write_text((json.dumps(row)+'\n')*2)
   with self.assertRaises(ValueError):RECON.load()
   self.manifest(p);bad={**row,'sha256':'0'*64};(p/'paths.jsonl').write_text(json.dumps(bad)+'\n')
   with self.assertRaises(ValueError):RECON.load()
   self.manifest(p);bad={**row,'bytes':1};(p/'paths.jsonl').write_text(json.dumps(bad)+'\n')
   with self.assertRaises(ValueError):RECON.verify_all(*RECON.load())
   self.manifest(p)
   with (p/'blobs-00001.bin').open('ab') as f:f.write(b'TRAILING')
   with self.assertRaises(ValueError):RECON.verify_all(*RECON.load())
   self.manifest(p);blob=p/'blobs-00001.bin';real=p/'real';blob.rename(real);blob.symlink_to(real)
   with self.assertRaises(ValueError):RECON.load()
   blob.unlink();real.rename(blob)
   self.manifest(p);target=p/'target';target.write_bytes(b'KEEP');link=p/'link';link.symlink_to(target)
   with self.assertRaises(ValueError):RECON.atomic_output(obj,link)
   parent=p/'parentlink';parent.symlink_to(p,target_is_directory=True)
   with self.assertRaises(ValueError):RECON.atomic_output(obj,parent/'new-file')
   bad={**obj,'sha256':'0'*64}
   with self.assertRaises(ValueError):RECON.atomic_output(bad,target)
   self.assertEqual(target.read_bytes(),b'KEEP');self.assertFalse(list(p.glob('.utxo-reconstruct-*')))
 def test_fresh_generation_reproducibility_and_omissions(self):
  with tempfile.TemporaryDirectory() as d:
   base=Path(d);r=base/'research';v=base/'validation';r.mkdir();v.mkdir();(r/'a.txt').write_text('public');(r/'.git').mkdir();(r/'empty').mkdir();(r/'directorylink').symlink_to(r/'empty',target_is_directory=True);(r/'filelink').symlink_to(r/'a.txt');(r/'unknown.jsonl').write_text('{"type":"unknown","data":"private"}\n')
   import os;os.mkfifo(r/'pipe')
   one=base/'one';two=base/'two';BUILD.build({'research':r,'validation':v},one);BUILD.build({'research':r,'validation':v},two)
   self.assertEqual({p.name:p.read_bytes() for p in one.iterdir()},{p.name:p.read_bytes() for p in two.iterdir()})
   original_copy=BUILD.shutil.copyfileobj
   def mutate(src,dst,*args):
    result=original_copy(src,dst,*args)
    if getattr(src,'name',None)==str(r/'a.txt'):(r/'a.txt').write_text('changed during capture')
    return result
   BUILD.shutil.copyfileobj=mutate
   try:BUILD.build({'research':r,'validation':v},base/'mutated')
   finally:BUILD.shutil.copyfileobj=original_copy
   self.assertTrue(any(json.loads(line)['reason']=='source-mutated-during-capture' for line in (base/'mutated/exclusions.jsonl').read_text().splitlines()))
   self.assertTrue(any(x.get('status')=='provenance-unavailable' for x in json.loads((one/'repository-provenance.json').read_text())))
   ex=[json.loads(line) for line in (one/'exclusions.jsonl').read_text().splitlines()];self.assertTrue(any(x['reason']=='non-regular-file' for x in ex));self.assertTrue(any(x['reason']=='file-symlink-not-followed' for x in ex));self.assertTrue(any(x['status']=='empty-or-unclassified' for x in map(json.loads,(one/'public-log-extractions.jsonl').read_text().splitlines())))
 def test_actual_filter_finalizer_chain_426(self):
  import os,subprocess
  with tempfile.TemporaryDirectory(dir=HERE) as d:
   base=Path(d);h=base/'archive';h.mkdir();r=base/'research';r.mkdir();v=base/'validation';v.mkdir()
   samples={'graph-turn.json':json.dumps({'directed':True,'nodes':[{'type':'turn.prompt','data':'PRIVATE'}]}).encode(),'graph-lifecycle.json':json.dumps({'directed':True,'nodes':[{'type':'lifecycle.started','data':'PRIVATE'}]}).encode(),'unknown.json':b'{"type":"session.started","content":"PRIVATE"}','nested.weird':b'{"wrapper":{"record_type":"unknown","other":"PRIVATE"}}','capture.weird':b'{"type":"thinking","data":"PRIVATE"}','oversized.log':b'{"type":"thinking","data":"'+b'x'*(51*1024*1024)+b'"}','changed-live.json':b'{"payload_type":"run.thought.delta","payload":{"text":"PRIVATE"}}','grok-text.txt':b'{"type":"text","data":"PUBLIC EXTRACT"}','public-graph.json':json.dumps({'directed':True,'nodes':[{'id':'x','label':'reasoning','source_file':'trace.jsonl'}]}).encode(),'public-schema.json':b'{"type":"object","properties":{"type":{"type":"string"}}}','package.json':b'{"name":"fixture","version":"1.0","type":"module"}','.gradle/cache.txt':b'cache'}
   packed=bytearray();objects=[];paths=[]
   for name,body in samples.items():
    dg=hashlib.sha256(body).hexdigest();z=gzip.compress(body,mtime=0);objects.append({'sha256':dg,'encoding':'gzip','parts':[{'blob':'blobs-00001.bin','offset':len(packed),'length':len(z)}]});packed.extend(z);paths.append({'root':'validation','path':name,'sha256':dg,'bytes':len(body)});p=v/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_text('{"public":"different live bytes"}')
   (h/'blobs-00001.bin').write_bytes(packed)
   for name,rows in [('objects.jsonl',objects),('paths.jsonl',paths),('exclusions.jsonl',[]),('public-log-extractions.jsonl',[])]: (h/name).write_text(''.join(json.dumps(x)+'\n' for x in rows))
   env={**os.environ,'UTXO_PUBLICATION_ARCHIVE':str(h),'UTXO_PUBLICATION_RESEARCH':str(r),'UTXO_PUBLICATION_VALIDATION':str(v)}
   run=lambda tool:subprocess.run([sys.executable,str(HERE/tool)],env=env,capture_output=True,text=True)
   self.assertNotEqual(run('finalize.py').returncode,0,'finalization without filter attestation must fail')
   result=run('filter.py');self.assertEqual(result.returncode,0,result.stderr)
   prior=(h/'paths.jsonl').read_bytes();(h/'paths.jsonl').write_bytes(prior+b'\n');self.assertNotEqual(run('finalize.py').returncode,0,'stale attestation must fail');(h/'paths.jsonl').write_bytes(prior)
   result=run('finalize.py');self.assertEqual(result.returncode,0,result.stderr)
   keep={json.loads(line)['path'] for line in (h/'paths.jsonl').read_text().splitlines()};self.assertEqual(keep,{'public-graph.json','public-schema.json','package.json'})
   RECON.BASE=h;RECON.verify_all(*RECON.load());extracts=[json.loads(line) for line in (h/'public-log-extractions.jsonl').read_text().splitlines()];self.assertTrue(any(x.get('public_bytes')==len(b'PUBLIC EXTRACT') for x in extracts));self.assertTrue((h/'FINALIZATION-GATE.json').is_file())
if __name__=='__main__':unittest.main(verbosity=2)
