"""Exact source assembly and expressly owner-authorised hash custody refresh, no JS/Git."""
from pathlib import Path,PurePosixPath
import json,hashlib,difflib,re
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017');D=Path(__file__).resolve().parent;BASE=R/'euc012-realm-error-contract-owner-assembly-20261019';P=R/'euc012-observed-error-contract-full-correction-muse-author-preparation-20261019-v2';J=R/'euc012-error-totality-muse-source-author-terminal-20261019';sha=lambda b:hashlib.sha256(b).hexdigest()
def put(n,b):p=D/n;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(b)
def emit(n,x):put(n,(json.dumps(x,indent=2)+'\n').encode())
def strictpairs(pairs):
 out={}
 for k,v in pairs:
  if k in out:raise ValueError('duplicate JSON key')
  out[k]=v
 return out
raw=(J/'TERMINAL-REVIEW.json').read_bytes();assert len(raw)==19249 and sha(raw)=='4dbdb4f35a530027f2a37ed5100bc640b069b20b69ca00538b190c6ecd4841b5';j=json.loads(raw,object_pairs_hook=strictpairs);patch=j['patch'];assert patch['baseTree']=='b8f8203b023aab5b56223dc8246c0c1cbf5ea4dc' and patch['afterFiles']==[] and patch['newFiles']==[]
put('custody/TERMINAL-OWNER-DRAFT.json',raw)
for n in ['TERMINAL-EXPORT-RECEIPT.json','CLEANUP.json'] :put('custody/'+n,(J/n).read_bytes())
def scoped(raw):
 p=PurePosixPath(raw);assert not p.is_absolute() and '..' not in p.parts and '.' not in p.parts and p.parts[:2]==('experiments','moriarty-eutxo');return str(p)
before={};sources={}
for x in json.loads((BASE/'ASSEMBLY.json').read_bytes())['files']:
 p=BASE/'after'/x['path'];b=p.read_bytes();assert len(b)==x['bytes'] and sha(b)==x['sha256'];before[x['path']]=b;sources[x['path']]=str(p)
assert len(before)==20
nativePin=next(x for x in json.loads((BASE/'ASSEMBLY.json').read_bytes())['files'] if x['path']=='experiments/moriarty-eutxo/src/native-caller.mjs')
after=dict(before);edits=[];grouped={}
for i,e in enumerate(patch['edits']):
 path=scoped(e['path']);assert path in before and set(e)=={'path','beforeText','afterText','expectedMatches'} and e['expectedMatches']==1
 old=e['beforeText'].encode('utf8');new=e['afterText'].encode('utf8');assert old and before[path].count(old)==1, (i,path,before[path].count(old))
 start=before[path].index(old);end=start+len(old);grouped.setdefault(path,[]).append((start,end,new));edits.append({'originalIndex':i,'path':path,'beforeStartByte':start,'beforeEndExclusive':end,'expectedMatches':1,'actualMatches':1,'beforeTextBytes':len(old),'beforeTextSha256':sha(old),'afterTextBytes':len(new),'afterTextSha256':sha(new)})
for path,ranges in grouped.items():
 ordered=sorted(ranges);assert all(x[1]<=y[0] for x,y in zip(ordered,ordered[1:]));b=before[path]
 for start,end,new in reversed(ordered):b=b[:start]+new+b[end:]
 after[path]=b
producer='experiments/moriarty-eutxo/src/compile-custody.mjs';producer_bytes=len(after[producer]);producer_sha=sha(after[producer]);api='experiments/moriarty-eutxo/tests/custody-module-loader.test.mjs';f01='experiments/moriarty-eutxo/tests/custody-required-test-identity.test.mjs';failure='experiments/moriarty-eutxo/tests/custody-failure-isolation.test.mjs';pinEdits=[]
owner_only=dict(after)
def refresh(path,old,new,label):
 old=old.encode();new=new.encode();b=after[path];assert b.count(old)==1;start=b.index(old);after[path]=b[:start]+new+b[start+len(old):];pinEdits.append({'path':path,'label':label,'beforeStartByteInSequentialRefresh':start,'beforeText':old.decode(),'afterText':new.decode(),'expectedMatches':1,'actualMatches':1,'hashInput':producer,'newProducerBytes':producer_bytes,'newProducerSha256':producer_sha})
refresh(api,"const EXPECTED_PRODUCER_BYTES = 31818;\nconst EXPECTED_PRODUCER_SHA256 = '5a3b6837a5b0f7315415e028f2c2fdf38947dbf252b893e3aeebcb2e90922831';",f"const EXPECTED_PRODUCER_BYTES = {producer_bytes};\nconst EXPECTED_PRODUCER_SHA256 = '{producer_sha}';",'API two exact fields at one authorised site')
refresh(f01,"current: { bytes: 31818, sha256: '5a3b6837a5b0f7315415e028f2c2fdf38947dbf252b893e3aeebcb2e90922831', path: join(root, 'src', 'compile-custody.mjs') },",f"current: {{ bytes: {producer_bytes}, sha256: '{producer_sha}', path: join(root, 'src', 'compile-custody.mjs') }},",'F01 current pair ONLY')
refresh(failure,"const EXPECTED_PRODUCER_BYTES = 31818;\nconst EXPECTED_PRODUCER_SHA256 = '5a3b6837a5b0f7315415e028f2c2fdf38947dbf252b893e3aeebcb2e90922831';",f"const EXPECTED_PRODUCER_BYTES = {producer_bytes};\nconst EXPECTED_PRODUCER_SHA256 = '{producer_sha}';",'failure test two externally frozen fields at one authorised site')
for path in [api,f01,failure]:
 undo=after[path]
 for e in reversed([x for x in pinEdits if x['path']==path]):
  new=e['afterText'].encode();assert undo.count(new)==1;undo=undo.replace(new,e['beforeText'].encode(),1)
 assert undo==owner_only[path]
for path,b in before.items():
 if path not in set(grouped)|{api,f01,failure}:assert after[path]==b
assert b"4e5b5a393ae9e11edc43ff68d47486f191f67317866553058bb65d650ff759e3" in after[f01]
rows=[]
for path,b in sorted(after.items()):
 put('after/'+path,b);put('before/'+path,before[path]);put('owner-exact-before-pinrefresh/'+path,owner_only[path])
 rows.append({'path':path,'bytes':len(b),'sha256':sha(b),'beforeBytes':len(before[path]),'beforeSha256':sha(before[path]),'ownerExactBeforePinRefreshSha256':sha(owner_only[path]),'source':sources[path],'changeClass':('OWNERexactuniqueedits+expresslyauthorisedpinrefresh' if path in [api,f01,failure] else 'OWNERexactuniqueedits' if path in grouped else 'UNCHANGEDexactsourcecopy')})
diff=[]
for path in sorted(after):
 if before[path]!=after[path]:diff.extend(difflib.unified_diff(before[path].decode().splitlines(True),after[path].decode().splitlines(True),fromfile='before/'+path,tofile='after/'+path))
put('ASSEMBLED-SOURCE.diff',''.join(diff).encode());emit('PIN-REFRESH.json',{'classification':'Owner expressly requested deterministic producer byte/SHA custody only, no semantic edits','producerPath':producer,'producerBytes':producer_bytes,'producerSha256':producer_sha,'edits':pinEdits,'siteCount':3,'fieldsCount':6,'undoRefreshEqualsExactOwnerEdits':True,'independentOldAndSevenFixturesUnchanged':True})
closure=[]
for path,b in sorted(after.items()):
 if path.endswith('.mjs') or path.endswith('.mjs.txt'):closure.append({'path':path,'sha256':sha(b),'importLines':[{'line':i,'text':line} for i,line in enumerate(b.decode().splitlines(),1) if re.match(r'^import\b',line)]})
emit('LITERAL-IMPORT-CLOSURE-INDEX.json',{'classification':'Literal source navigation only, not selected graph/effects judgment','files':closure,'candidateRuntimeUNRUN':True})
report={'classification':'MECHANICAL OWNER SOURCE ASSEMBLY/PIN CUSTODY ONLY; no Git/product/runtime/security judgment','baseTree':patch['baseTree'],'baseCommit':None,'baselineAssembly':str(BASE/'ASSEMBLY.json'),'authorDraftBytes':len(raw),'authorDraftSha256':sha(raw),'files':rows,'fileCount':len(rows),'totalAfterBytes':sum(x['bytes'] for x in rows),'ownerEdits':edits,'nativeCallerWholeBaselinePin':nativePin,'allBeforeTextUniqueAndDisjoint':True,'ownerEditsPerFile':{path:len(ranges) for path,ranges in grouped.items()},'ownerNewFiles':0,'pinRefresh':'PIN-REFRESH.json','frozenOldAndSevenFixturesUnchanged':True,'newSourceSyntaxAndRuntime':'UNRUN; no Node executed','sourceAcceptanceOrExecutionAdmission':False,'remaining':'All13/fullEUC012, meaningful6artifact/manifest/credential/realstaleconsumer, F13/GUARD_SHAPE/nativecaller and full120/public64/parent remainOPEN'};emit('ASSEMBLY.json',report)
print(json.dumps({'afterFiles':len(rows),'producerBytes':producer_bytes,'producerSha256':producer_sha,'edits':len(edits),'changedFiles':len(grouped),'pinSites':len(pinEdits),'assemblySha256':sha((D/'ASSEMBLY.json').read_bytes()),'providerProductGitRuntimeCalls':0},indent=2))
