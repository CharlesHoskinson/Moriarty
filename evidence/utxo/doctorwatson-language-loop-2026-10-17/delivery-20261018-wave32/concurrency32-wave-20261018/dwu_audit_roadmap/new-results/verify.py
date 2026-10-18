from pathlib import Path
import json,hashlib,copy,importlib.util
D=Path(__file__).resolve().parent
W=Path('/home/charl/Moriarty/.worktrees/utxo')
def read(p):return json.loads(Path(p).read_text())
def pointer(doc,path):
 parts=[p.replace('~1','/').replace('~0','~') for p in path.lstrip('/').split('/')]
 cur=doc
 for part in parts:cur=cur[int(part)] if isinstance(cur,list) else cur[part]
 return cur
manifest=read(D/'MANIFEST.json')
for x in manifest['outputs']:
 b=(D/x['path']).read_bytes();assert hashlib.sha256(b).hexdigest()==x['sha256'] and len(b)==x['bytes'],x['path']
pub=read(D/'PUBLICATION-MANIFEST.json')
for x in pub['members']:
 b=Path(x['preparedLocalPath']).read_bytes();assert hashlib.sha256(b).hexdigest()==x['sha256'] and len(b)==x['bytes'],x['destination']
original=read(W/'openspec/changes/utxo-doctorwatson/current-evidence.json');candidate=copy.deepcopy(original)
for op in read(D/'current-evidence.partial-refs.patch.json'):
 if op['op']=='test':assert pointer(candidate,op['path'])==op['value'],op['path']
 else:
  assert op['op']=='add' and op['path'].endswith('/validation/evidenceRefs/-')
  parent=pointer(candidate,op['path'][:-2]);parent.append(op['value'])
import jsonschema
schema=read(W/'openspec/changes/utxo-doctorwatson/evidence.schema.json');jsonschema.Draft202012Validator(schema).validate(candidate)
spec=importlib.util.spec_from_file_location('metadata_reader',W/'models/utxo/qa/check.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
class Overlay(m.Files):
 def read(self,name):
  prefix=pub['destinationRoot']+'/'
  if name.startswith(prefix):return (D/'publication'/name[len(prefix):]).read_bytes()
  return super().read(name)
baseline=read(W/'docs/utxo/doctorwatson/EARS-BASELINE.json');base=m.inspect(m.Files(W),baseline,original);after=m.inspect(Overlay(W),baseline,candidate)
assert base['integrity']==after['integrity']=={'status':'ok','issues':[]}
assert len(after['requirements'])==148 and len(after['families'])==20 and all(r['gate']=='open' and r['acceptance']['status']=='open' for r in after['requirements'])
assert sum(len(r['scenarios']) for g in baseline['groups'] for r in g['requirements'])==292
for a,b in zip(original['rows'],candidate['rows']):
 for k in a:
  if k!='validation':assert a[k]==b[k]
 assert a['validation']['status']==b['validation']['status']
 assert b['validation']['evidenceRefs'][:len(a['validation']['evidenceRefs'])]==a['validation']['evidenceRefs']
assert original['families']==candidate['families']
partial=read(D/'publication/PARTIAL-OBSERVATIONS.json');assert len(partial['retained']['roadmapIds'])==69 and partial['retained']['allParentRoadmapExitsOpen']
for n in ['SOURCE-PROGRESS','ASSESSMENT']:
 patch=read(D/(n+'.observation.patch.json'));assert len(patch)==1 and patch[0]['op']=='add' and patch[0]['path']=='/concurrency32AdditionalResults20261018'
 assert patch[0]['value']['observations']==partial['observations']
print('PASS: manifests/publication hashes; actualschema; actualmetadata reader integrityok;148/292/20/69 retained;allgatesopen;onlypartialrefs appended')
