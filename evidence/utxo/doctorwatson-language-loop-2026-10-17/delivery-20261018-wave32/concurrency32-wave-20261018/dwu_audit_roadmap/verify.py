from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent
s=json.loads((p/'SOURCE-PINS.json').read_text())
for v in s['actualReceiptAndSnapshotPins']:
 b=Path(v['path']).read_bytes(); assert hashlib.sha256(b).hexdigest()==v['sha256'] and len(b)==v['bytes'],v['path']
d=json.loads((p/'OBSERVATION-DELTA.json').read_text());r=json.loads((p/'RETAINED-INVENTORY-CHECKS.json').read_text());assert(r['requirements'],r['scenarios'],r['families'],len(r['roadmapIds']))==(148,292,20,69)
assert r['all69ParentsOpen'] and r['newWholeOriginalClausePasses']==0
for n in ['SOURCE-PROGRESS','ASSESSMENT']:
 patch=json.loads((p/(n+'.observation.patch.json')).read_text());assert len(patch)==1 and patch[0]['op']=='add' and patch[0]['path']=='/concurrency32ObservedChildren20261018' and patch[0]['value']==d['proposedSharedValue']
q=d['proposedSharedValue']['q1FreshAuditTimeline'];assert q['terminalExitCode']==-9 and q['decision']=='NO_EXPORT' and q['terminalCleanup']['ownedGroupRemaining'] and not q['terminalCleanup']['captureReachedEOF'] and q['laterProcessCensus']['rows']==[]
assert all(not l['wholeOriginalClauseOrScenarioPassProposed'] for l in d['exactClauseLinks'])
print('PASS: receipt hashes; two append-only patches;148/292/20/69 retained; Q1 timings separate; no acceptance promotion')
