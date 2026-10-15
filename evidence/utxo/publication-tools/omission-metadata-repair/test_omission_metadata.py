"""Uninventoried omission metadata: actual filter->finalize->reconstruct chain, verify invariants and adoption."""
import unittest,tempfile,json,gzip,hashlib,os,sys,shutil,subprocess
from pathlib import Path
from importlib.machinery import SourceFileLoader
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent
HELPERS=Path(os.environ.get('OMISSION_HELPERS',HERE)).resolve()
FROZEN=Path(os.environ.get('OMISSION_FROZEN','/home/charl/utxo-publication-build/omission-metadata-frozen-inputs')).resolve()
RECON=SourceFileLoader('omission_reconstruction',str(HELPERS/'reconstruct.py')).load_module()
PRIVATE='SYNTHETIC-PRIVATE-CAPTURE-BODY'
LATE='publication-archive-review-426/review-private-events.jsonl'
REASON='new path after capture inventory; publication deferred to explicit frozen public supplement'
UNKNOWN={'bytes':551565,'path':LATE,'public_sha256':None,'reason':REASON,'root':'validation','sha256':None,'status':'source-uninventoried'}
# Run a helper with an audit hook that refuses any open() beneath the guarded paths.
GUARD='''import os,sys,runpy
roots=[os.path.abspath(x) for x in os.environ['OMISSION_GUARD'].split(os.pathsep)]
def hook(event,args):
 if event=='open' and args and isinstance(args[0],(str,bytes,os.PathLike)):
  p=os.path.abspath(os.fsdecode(args[0]))
  if any(p==r or p.startswith(r+os.sep) for r in roots):raise PermissionError('guarded uninventoried source opened')
tool=os.path.abspath(sys.argv[1]);sys.argv=sys.argv[1:];sys.path[0]=os.path.dirname(tool)
sys.addaudithook(hook);runpy.run_path(tool,run_name='__main__')'''
def sha(b):return hashlib.sha256(b).hexdigest()
def rows(p):return [json.loads(l) for l in p.read_text().splitlines()]
def corpus(base,late_source=True):
 h=base/'archive';r=base/'research';v=base/'validation'
 for d in [h,r,v]:d.mkdir()
 samples={'public.txt':b'PUBLIC ORIGINAL\n','capture.jsonl':(json.dumps({'type':'text','data':'PUBLIC EXTRACT'})+'\n'+json.dumps({'type':'thinking','data':PRIVATE})+'\n').encode()}
 packed=bytearray();objects=[];paths=[]
 for name,body in samples.items():
  z=gzip.compress(body,mtime=0);objects.append({'sha256':sha(body),'encoding':'gzip','parts':[{'blob':'blobs-00001.bin','offset':len(packed),'length':len(z)}]});packed.extend(z);paths.append({'root':'research','path':name,'sha256':sha(body),'bytes':len(body)});(r/name).write_bytes(body)
 (h/'blobs-00001.bin').write_bytes(packed);late=v/LATE
 if late_source:late.parent.mkdir(parents=True);late.write_text(json.dumps({'type':'thinking','data':PRIVATE})+'\n')
 # The late row was never inventoried: no recorded digest, only its omission origin.
 ex=[{'root':'validation','path':LATE,'bytes':551565,'reason':REASON}]
 for name,rs in [('objects.jsonl',objects),('paths.jsonl',paths),('exclusions.jsonl',ex),('public-log-extractions.jsonl',[])]:(h/name).write_text(''.join(json.dumps(x)+'\n' for x in rs))
 return h,r,v,late
def run(tool,h,r,v,where=HELPERS,args=(),guard=()):
 env={**os.environ,'UTXO_PUBLICATION_ARCHIVE':str(h),'UTXO_PUBLICATION_RESEARCH':str(r),'UTXO_PUBLICATION_VALIDATION':str(v),'PYTHONDONTWRITEBYTECODE':'1'};env.pop('UTXO_PUBLIC_SOURCE_PROFILE',None)
 if guard:env['OMISSION_GUARD']=os.pathsep.join(map(str,guard));cmd=[sys.executable,'-B','-c',GUARD,str(where/tool),*map(str,args)]
 else:cmd=[sys.executable,'-B',str(where/tool),*map(str,args)]
 return subprocess.run(cmd,env=env,capture_output=True,text=True,cwd=h.parent)
def verify(h,where=HELPERS):
 shutil.copyfile(where/'reconstruct.py',h/'reconstruct.py');return subprocess.run([sys.executable,'-B',str(h/'reconstruct.py'),'--verify-all'],capture_output=True,text=True,cwd=h.parent)
def decoded(h):
 out=b''
 for o in rows(h/'objects.jsonl'):
  data=b''.join((h/p['blob']).read_bytes()[p['offset']:p['offset']+p['length']] for p in o['parts']);out+=gzip.decompress(data)
 return out
class UninventoriedChain(unittest.TestCase):
 def chain(self,late_source):
  with tempfile.TemporaryDirectory(dir=HERE) as d:
   h,r,v,late=corpus(Path(d),late_source)
   x=run('filter.py',h,r,v);self.assertEqual(x.returncode,0,x.stderr);x=run('finalize.py',h,r,v);self.assertEqual(x.returncode,0,x.stderr)
   x=verify(h);self.assertEqual(x.returncode,0,x.stderr);self.assertIn('verified 2 unique objects and 1 original paths',x.stdout)
   public=rows(h/'public-log-extractions.jsonl');row=[x for x in public if x['path']==LATE]
   self.assertEqual(row,[UNKNOWN]);self.assertNotIn('public_bytes',row[0])
   hashed=[x for x in public if x['path']=='capture.jsonl'];self.assertEqual(len(hashed),1);self.assertRegex(hashed[0]['sha256'],'^[0-9a-f]{64}$');self.assertEqual(hashed[0]['public_bytes'],len(b'PUBLIC EXTRACT'))
   self.assertEqual(rows(h/'exclusions.jsonl')[-1],{'root':'validation','path':LATE,'bytes':551565,'reason':REASON})
   self.assertNotIn(PRIVATE.encode(),decoded(h));self.assertFalse(any(PRIVATE.encode() in p.read_bytes() for p in h.iterdir() if p.is_file()))
 def test_late_uninventoried_omission_reconstructs(self):self.chain(True)
 def test_unavailable_uninventoried_omission_reconstructs(self):self.chain(False)
 def test_uninventoried_source_is_never_opened(self):
  with tempfile.TemporaryDirectory(dir=HERE) as d:
   h,r,v,late=corpus(Path(d));x=run('filter.py',h,r,v,guard=[late]);self.assertEqual(x.returncode,0,x.stderr[-400:])
 def test_digest_bearing_rows_unchanged_from_frozen_filter(self):
  if not (FROZEN/'filter.py').is_file():self.skipTest('frozen helpers unavailable')
  with tempfile.TemporaryDirectory(dir=HERE) as d:
   out={}
   for label,where in [('frozen',FROZEN),('candidate',HELPERS)]:
    (Path(d)/label).mkdir();h,r,v,late=corpus(Path(d)/label)
    for tool in ['filter.py','finalize.py']:x=run(tool,h,r,v,where);self.assertEqual(x.returncode,0,x.stderr)
    out[label]={n:(h/n).read_bytes() for n in ['paths.jsonl','exclusions.jsonl','objects.jsonl','SUMMARY.json','blobs-00001.bin']}
    out[label]['public']=[x for x in rows(h/'public-log-extractions.jsonl') if x['path']!=LATE]
   self.assertEqual(out['frozen'],out['candidate'])
class VerifyInvariants(unittest.TestCase):
 def setUp(self):
  self.tmp=tempfile.TemporaryDirectory();self.p=p=Path(self.tmp.name);RECON.BASE=p
  self.body=b'PUBLIC EXTRACT';z=gzip.compress(self.body,mtime=0);(p/'blobs-00001.bin').write_bytes(z)
  self.obj={'sha256':sha(self.body),'encoding':'gzip','parts':[{'blob':'blobs-00001.bin','offset':0,'length':len(z)}]}
  self.path={'root':'research','path':'public.txt','sha256':sha(self.body),'bytes':len(self.body)}
  self.hashed={'root':'validation','path':'capture.jsonl','sha256':'1'*64,'status':'public-text','public_sha256':sha(self.body),'public_bytes':len(self.body)}
 def tearDown(self):self.tmp.cleanup()
 def write(self,extracts,objects=None,paths=None):
  for name,rs in [('objects.jsonl',[self.obj] if objects is None else objects),('paths.jsonl',[self.path] if paths is None else paths),('public-log-extractions.jsonl',extracts)]:(self.p/name).write_text(''.join(json.dumps(x)+'\n' for x in rs))
 def accepts(self,extracts,**kw):self.write(extracts,**kw);return RECON.verify_all(*RECON.load())
 def refuses(self,extracts,**kw):
  self.write(extracts,**kw)
  with self.assertRaises(ValueError,msg=json.dumps(extracts)):RECON.verify_all(*RECON.load())
 def test_explicit_unknown_digest_omission_accepted(self):
  self.assertEqual(self.accepts([self.hashed,UNKNOWN]),{sha(self.body):len(self.body)})
  self.assertEqual(self.accepts([UNKNOWN],paths=[self.path]),{sha(self.body):len(self.body)})
 def test_unknown_digest_with_public_payload_refused(self):
  for bad in [{**UNKNOWN,'public_sha256':sha(self.body)},{**UNKNOWN,'public_sha256':sha(self.body),'public_bytes':len(self.body)},{**UNKNOWN,'public_bytes':0},{**UNKNOWN,'public_sha256':'0'*64}]:self.refuses([bad],paths=[self.path])
 def test_unknown_digest_must_be_explicit_auditable_uninventoried(self):
  actual={k:v for k,v in UNKNOWN.items() if k!='sha256'};actual['status']='source-changed-since-recorded-digest'
  implicit={k:v for k,v in UNKNOWN.items() if k!='sha256'};nopublic={k:v for k,v in UNKNOWN.items() if k!='public_sha256'};noreason={k:v for k,v in UNKNOWN.items() if k!='reason'}
  for bad in [actual,implicit,nopublic,noreason,{**UNKNOWN,'reason':''},{**UNKNOWN,'reason':7},{**UNKNOWN,'status':'source-changed-since-recorded-digest'},{**UNKNOWN,'status':'source-unavailable'},{**UNKNOWN,'status':None},{**UNKNOWN,'root':'research','path':'public.txt'}]:self.refuses([bad])
 def test_invalid_digests_refused(self):
  for value in ['','0'*63,'0'*65,'A'*64,'g'*64,123,False,['0'*64]]:
   self.refuses([{**self.hashed,'sha256':value}]);self.refuses([{**UNKNOWN,'sha256':value}])
 def test_hashed_extraction_path_and_existing_negatives_unchanged(self):
  self.assertEqual(self.accepts([self.hashed],paths=[]),{sha(self.body):len(self.body)})
  self.refuses([{**self.hashed,'public_sha256':'2'*64}],paths=[])
  self.refuses([{**self.hashed,'public_bytes':1}],paths=[])
  self.refuses([self.hashed,self.hashed],paths=[]);self.refuses([UNKNOWN,UNKNOWN]);self.refuses([UNKNOWN,{**self.hashed,'path':LATE}],paths=[])
  for path in ['../escape.jsonl','a/../../escape.jsonl','/etc/passwd']:self.refuses([{**UNKNOWN,'path':path}]);self.refuses([{**self.hashed,'path':path}],paths=[])
  for root in ['other',None]:self.refuses([{**UNKNOWN,'root':root}])
  self.refuses([UNKNOWN],paths=[])
  orphan=b'ORPHAN';z=gzip.compress(orphan,mtime=0)
  with (self.p/'blobs-00001.bin').open('ab') as f:f.write(z)
  extra={'sha256':sha(orphan),'encoding':'gzip','parts':[{'blob':'blobs-00001.bin','offset':self.obj['parts'][0]['length'],'length':len(z)}]}
  self.refuses([UNKNOWN],objects=[self.obj,extra])
  self.refuses([UNKNOWN])
  (self.p/'blobs-00001.bin').write_bytes(gzip.compress(self.body,mtime=0))
  self.refuses([UNKNOWN],objects=[{**self.obj,'parts':[{**self.obj['parts'][0],'length':self.obj['parts'][0]['length']+1}]}])
  (self.p/'blobs-00002.bin').write_bytes(b'');self.refuses([UNKNOWN]);(self.p/'blobs-00002.bin').unlink()
  self.assertEqual(self.accepts([UNKNOWN]),{sha(self.body):len(self.body)})
class Adoption(unittest.TestCase):
 def setUp(self):
  if not (FROZEN/'filter.py').is_file():self.skipTest('frozen helpers unavailable')
  self.tmp=tempfile.TemporaryDirectory(dir=HERE);self.base=Path(self.tmp.name)
 def tearDown(self):self.tmp.cleanup()
 def finalized(self,label,late_source=True):
  (self.base/label).mkdir();h,r,v,late=corpus(self.base/label,late_source)
  for tool in ['filter.py','finalize.py']:x=run(tool,h,r,v,FROZEN);self.assertEqual(x.returncode,0,x.stderr)
  return h,r,v
 def adopt(self,h,r,v,gate=None,expect=1,recon=None):
  recon=recon or HELPERS/'reconstruct.py';gate=gate or sha((h/'FINALIZATION-GATE.json').read_bytes())
  return run('adopt_omission_metadata.py',h,r,v,HERE,['--archive',h,'--gate-sha256',gate,'--expect-rows',expect,'--reconstruct',recon,'--reconstruct-sha256',sha((HELPERS/'reconstruct.py').read_bytes())],guard=[r,v])
 def snapshot(self,h):return {p.name:p.read_bytes() for p in h.iterdir() if p.is_file()}
 def regate(self,h):
  gate=json.loads((h/'FINALIZATION-GATE.json').read_text());gate['final_manifest_sha256']={n:sha((h/n).read_bytes()) for n in gate['final_manifest_sha256']};(h/'FINALIZATION-GATE.json').write_text(json.dumps(gate,sort_keys=True,indent=2)+'\n')
 def test_actual_shape_reproduced_then_metadata_only_adoption(self):
  for late_source,prior in [(True,'source-changed-since-recorded-digest'),(False,'source-unavailable')]:
   h,r,v=self.finalized('actual-'+prior,late_source);row=[x for x in rows(h/'public-log-extractions.jsonl') if x['path']==LATE]
   self.assertEqual(row,[{k:x for k,x in UNKNOWN.items() if k!='sha256'}|{'status':prior}])
   x=verify(h,FROZEN);self.assertEqual(x.returncode,1);self.assertIn('ValueError: invalid extraction source digest',x.stderr)
   before=self.snapshot(h);gate=json.loads(before['FINALIZATION-GATE.json']);x=self.adopt(h,r,v);self.assertEqual(x.returncode,0,x.stderr);after=self.snapshot(h)
   self.assertEqual({n for n in after if before.get(n)!=after[n]},{'public-log-extractions.jsonl','reconstruct.py','OMISSION-METADATA-ADOPTION.json'})
   self.assertEqual(sha(before['public-log-extractions.jsonl']),gate['final_manifest_sha256']['public-log-extractions.jsonl'])
   old=before['public-log-extractions.jsonl'].splitlines(keepends=True);new=after['public-log-extractions.jsonl'].splitlines(keepends=True);changed=[i for i in range(len(old)) if old[i]!=new[i]]
   self.assertEqual(len(old),len(new));self.assertEqual(len(changed),1);self.assertEqual(json.loads(new[changed[0]]),{**UNKNOWN,'status_before_metadata_adoption':prior})
   receipt=json.loads(after['OMISSION-METADATA-ADOPTION.json'])
   self.assertEqual(receipt['finalization_gate_sha256'],sha(before['FINALIZATION-GATE.json']));self.assertEqual(receipt['filter_attestation_sha256'],sha(before['FILTER-ATTESTATION.json']))
   self.assertEqual((receipt['manifest_sha256_before'],receipt['manifest_sha256_after']),(sha(before['public-log-extractions.jsonl']),sha(after['public-log-extractions.jsonl'])))
   self.assertEqual(receipt['unchanged_manifest_sha256'],{n:gate['final_manifest_sha256'][n] for n in ['paths.jsonl','exclusions.jsonl','objects.jsonl']})
   self.assertEqual((receipt['reconstruct_sha256_before'],receipt['reconstruct_sha256_after']),(sha((FROZEN/'reconstruct.py').read_bytes()),sha((HELPERS/'reconstruct.py').read_bytes())))
   self.assertEqual(receipt['adoption_tool_sha256'],sha((HERE/'adopt_omission_metadata.py').read_bytes()))
   for k,want in [('original_attestations_retained_unchanged',True),('original_attestations_cover_adopted_manifest',False),('source_bodies_read',False),('publication_exception',False),('source_approval',False),('native_approval',False)]:self.assertIs(receipt[k],want,k)
   # The receipt alone restores the exact gate-pinned original manifest bytes.
   restored=list(new)
   for c in receipt['changed_lines']:
    self.assertEqual(restored[c['line']-1],(c['after']+'\n').encode());restored[c['line']-1]=(c['before']+'\n').encode()
   self.assertEqual(sha(b''.join(restored)),gate['final_manifest_sha256']['public-log-extractions.jsonl'])
   x=subprocess.run([sys.executable,'-B',str(h/'reconstruct.py'),'--verify-all'],capture_output=True,text=True);self.assertEqual(x.returncode,0,x.stderr)
   self.assertNotEqual(run('finalize.py',h,r,v,FROZEN).returncode,0,'original attestation must not cover adopted bytes')
   again=self.snapshot(h);self.assertNotEqual(self.adopt(h,r,v,gate=receipt['finalization_gate_sha256']).returncode,0);self.assertEqual(self.snapshot(h),again)
 def test_adoption_refusals_leave_generation_unchanged(self):
  def refused(h,r,v,**kw):
   before=self.snapshot(h);x=self.adopt(h,r,v,**kw);self.assertNotEqual(x.returncode,0,x.stdout);self.assertEqual(self.snapshot(h),before);return x
  h,r,v=self.finalized('pins')
  refused(h,r,v,gate='0'*64);refused(h,r,v,expect=2);refused(h,r,v,expect=0)
  wrong=self.base/'wrong-reconstruct.py';wrong.write_bytes((HELPERS/'reconstruct.py').read_bytes()+b'\n');refused(h,r,v,recon=wrong)
  link=self.base/'link-reconstruct.py';link.symlink_to(HELPERS/'reconstruct.py');refused(h,r,v,recon=link)
  p=h/'paths.jsonl';original=p.read_bytes();p.write_bytes(original+b'\n');refused(h,r,v);p.write_bytes(original)
  a=h/'FILTER-ATTESTATION.json';original=a.read_bytes();a.write_bytes(original+b'\n');refused(h,r,v);a.write_bytes(original)
  public=h/'public-log-extractions.jsonl';exclusions=h/'exclusions.jsonl';base_public=public.read_bytes();base_ex=exclusions.read_bytes()
  def mutate(row_change=None,ex_change=None):
   rs=rows(public);es=rows(exclusions)
   for x in rs:
    if x['path']==LATE and row_change:row_change(x)
   for x in es:
    if x['path']==LATE and ex_change:ex_change(x)
   public.write_text(''.join(json.dumps(x,sort_keys=True)+'\n' for x in rs));exclusions.write_text(''.join(json.dumps(x,sort_keys=True)+'\n' for x in es));self.regate(h)
   refused(h,r,v,gate=sha((h/'FINALIZATION-GATE.json').read_bytes()));public.write_bytes(base_public);exclusions.write_bytes(base_ex);self.regate(h)
  mutate(ex_change=lambda x:x.update(sha256='1'*64))
  mutate(ex_change=lambda x:x.update(reason='other'))
  mutate(row_change=lambda x:x.update(public_sha256='1'*64))
  mutate(row_change=lambda x:x.update(public_bytes=0))
  mutate(row_change=lambda x:x.update(status='public-text'))
  mutate(row_change=lambda x:x.update(reason=''))
  mutate(row_change=lambda x:x.update(sha256=None,status='source-uninventoried'))
  x=self.adopt(h,r,v);self.assertEqual(x.returncode,0,x.stderr)
if __name__=='__main__':unittest.main(verbosity=2)
