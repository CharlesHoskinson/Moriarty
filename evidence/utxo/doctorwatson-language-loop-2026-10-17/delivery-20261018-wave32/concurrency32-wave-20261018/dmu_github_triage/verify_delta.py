from pathlib import Path
import hashlib,json
D=Path(__file__).resolve().parent
sha=lambda b:hashlib.sha256(b).hexdigest()
S=json.loads((D/'CURRENT-TRIAGE.json').read_text())
B=json.loads((D/'work-items-before.json').read_text())
A=json.loads((D/'work-items-after.json').read_text())
assert sha((D/'work-items-before.json').read_bytes())==S['trackerGuard']['beforeSha256']
assert sha((D/'work-items-after.json').read_bytes())==S['trackerGuard']['afterSha256']
assert A['items'][:20]==B['items'][:20]
assert [x['id'] for x in A['items'][:-1]]==[x['id'] for x in B['items']]
assert A['items'][-1]['id']=='GH-PR231'
for old in B['items']:
 new=next(x for x in A['items'] if x['id']==old['id'])
 if old['id']!='GH-PR227': assert old==new
 else:
  for k,v in old.items():
   if k!='currentRemoteObservation': assert new[k]==v
  assert new['acceptance']==old['acceptance']=='open'
  assert new['sourceObservationHistory'][-1]==old['currentRemoteObservation']
for k,v in B.items():
 if k not in ['items','counts']: assert A[k]==v
assert sum(A['originalInventoriesUnchanged'].values())==148
assert A['counts']=={'issues':15,'pullRequests':11,'total':26}
for rec in json.loads((D/'CAPTURE-MANIFEST.json').read_text())['records']:
 assert sha((D/rec['path']).read_bytes())==rec['sha256'],rec['path']
for rec in json.loads((D/'SOURCE-MANIFEST.json').read_text()):
 raw=(D/rec['completeBody']).read_bytes()
 assert sha(raw)==rec['sha256'],rec['path']
 assert hashlib.sha1(b'blob '+str(len(raw)).encode()+b'\0'+raw).hexdigest()==rec['gitBlob'],rec['path']
for rec in json.loads((D/'RECEIPT-MANIFEST.json').read_text()):
 assert sha((D/rec['copy']).read_bytes())==rec['sha256'],rec['copy']
for p in S['openPullRequests']:
 cap=json.loads((D/p['bodyCapture']).read_text())
 assert cap['headRefOid']==p['head'] and cap['baseRefOid']==p['base']
 assert cap['state']=='OPEN' and cap['isDraft']
 assert sha(cap['body'].encode())==p['bodySha256']
 rest=json.loads((D/f"captures/pr-{p['number']}-rest-membership.json").read_text())
 assert len(p['changedPaths'])==rest['changed_files']
print('PASS: source/receipt/API pins; original20/merged/duplicate history; only227 observation refreshed and231 appended;148retained EARS.')
