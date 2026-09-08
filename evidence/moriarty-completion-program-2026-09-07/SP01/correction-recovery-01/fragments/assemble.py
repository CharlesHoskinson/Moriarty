from pathlib import Path
import json, hashlib, shutil
S=Path(__file__).resolve().parent
BASE=Path('/home/charl/.local/state/moriarty/sp01-full-challenge-map-20260908/candidate-01/experiments/moriarty-language/spec/successor')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
read=lambda p:json.loads(p.read_text())
M=read(BASE/'challenge-map.json');T=read(BASE/'challenge-traces.json')
old_rows={x['qualifiedId']:x for x in M['challenges']+M['compositionOperators']}
old_traces={x['id']:x for x in T['traces']};old_mutations={x['id']:x for x in T['mutations']}
rows={};traces={};mutations={};fragments={};inputs={};files={}
for g in ['credit','markets','workflows','composition']:
 b=read(S/g/'binding.json');w=Path(b['worktree']);initial=read(S/g/'design-worker-receipt.json'); receipt=initial
 for suffix in ['completion-worker','recovery-worker']:
  if (S/g/(suffix+'-receipt.json')).exists():receipt=read(S/g/(suffix+'-receipt.json'))
 assert receipt['exitCode']==0 and not receipt['unauthorizedGitActivity'] and not receipt['storageStop'],(g,'worker not successful')
 assert receipt.get('modelUsage',{}).get('grok-4.6-build'),(g,'no actual requested model evidence')
 assert sha(S/g/'input-packet.json')==b['inputs']['input-packet.json']
 packet=read(S/g/'input-packet.json');p=w/b['ownedFiles'][0];f=read(p);fragments[g]=f
 assert f['group']==g and f['status']=='specified-only',g
 assert sum((w/x).stat().st_size for x in b['ownedFiles'])<=524288,(g,'owned limit')
 assert len({r['qualifiedId'] for r in f['rows']})==len(f['rows'])
 assert {r['qualifiedId'] for r in f['rows']}=={r['qualifiedId'] for r in packet['rows']},(g,'row coverage')
 assert {t['id'] for t in f['traces']}=={t['id'] for t in packet['traces']},(g,'trace coverage')
 assert {t['id'] for t in f['mutations']} >= {t['id'] for t in packet['mutations']},(g,'mutation coverage')
 assert f['verification'],(g,'no independent derivations retained')
 for r in f['rows']:
  key=r['qualifiedId'];assert key not in rows and key in old_rows
  old=old_rows[key]
  for field in ['id','family','source','positiveTraceId']:
   assert r[field]==old[field],(g,key,field,'protected binding changed')
  assert r['notSemanticFreeze'] and r['notImplementation'] and r['notMechanizedProof'] and r['notConformance']
  rows[key]=r
 for t in f['traces']:
  assert t['id'] not in traces;traces[t['id']]=t
 for t in f['mutations']:
  assert t['id'] not in mutations;mutations[t['id']]=t
 inputs[g]={'packetSha256':sha(S/g/'input-packet.json'),'fragmentSha256':sha(p),'receiptSha256':sha(S/g/'design-worker-receipt.json')}
assert set(rows)==set(old_rows) and set(traces)==set(old_traces) and set(mutations)>=set(old_mutations)
def rewrite_refs(value,group):
 if isinstance(value,dict):
  return {k:('/financialFragments/'+group+v if k in ['ref','oracleRef'] and isinstance(v,str) and v.startswith('/') else rewrite_refs(v,group)) for k,v in value.items()}
 if isinstance(value,list):return [rewrite_refs(v,group) for v in value]
 return value
for g,f in fragments.items():
 for row in f['rows']:rows[row['qualifiedId']]=rewrite_refs(row,g)
 for row in f['traces']:traces[row['id']]=rewrite_refs(row,g)
 for row in f['mutations']:mutations[row['id']]=rewrite_refs(row,g)
M['financialFragments']={g:rewrite_refs(f,g) for g,f in fragments.items()}
T['financialFragments']=M['financialFragments']
M['challenges']=[rows[x['qualifiedId']] for x in M['challenges']]
M['compositionOperators']=[rows[x['qualifiedId']] for x in M['compositionOperators']]
M['commonAcceptance']=fragments['composition']['commonAcceptance']
M['correctionProvenance']={'status':'specified-only-awaiting-independent-review','fragments':inputs,'predecessorCandidate':'649335fc8f1e97701ef1c4e6ed5619e4c70046a83fdb1f6afef272adad076331'}
T['traces']=[traces[x['id']] for x in T['traces']]
T['mutations']=[mutations[x['id']] for x in T['mutations']]+[m for key,m in mutations.items() if key not in old_mutations]
# Baseline complete inventories must survive byte-canonical comparison unchanged.
original=read(BASE/'challenge-map.json')
for field in ['inputs','inventories','candidateProducts','atomicCoreRetention','coverage','closureTasks']:
 assert M[field]==original[field],field
O=S/'assembled-candidate-02';O.mkdir(exist_ok=False)
for name,obj in [('challenge-map.json',M),('challenge-traces.json',T),('theorem-ledger.json',fragments['composition']['theoremLedger'])]:
 p=O/name;p.write_text(json.dumps(obj,indent=2,ensure_ascii=False)+'\n');files[name]=sha(p)
for g,f in fragments.items():
 p=O/('fragment-'+g+'.json');p.write_text(json.dumps(f,indent=2,ensure_ascii=False)+'\n');files[p.name]=sha(p)
report={'status':'assembled-not-accepted','scope':'Mechanical exact-ID replacement and unchanged inventory/source-binding checks only. Economic validity and theorem completeness require independent review. No execution/proof/public claims.','files':files,'candidateSha256':hashlib.sha256(json.dumps(files,sort_keys=True,separators=(',',':')).encode()).hexdigest(),'rows':len(rows),'traces':len(traces),'mutations':len(mutations),'sourceFragments':inputs}
(S/'assembly-freeze.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
