import copy,hashlib,importlib.util,json,os,stat,unittest,io
from pathlib import Path
R=Path('/home/charl/Moriarty');O=R/'deliverables/language-to-ledger-2026-09-12/sprints/LL01'
def read(p):return json.loads(Path(p).read_bytes())
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def mod(name,p):
 s=importlib.util.spec_from_file_location(name,p);m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
m=mod('current13',R/'.moriarty-dev/k-macro05-trace106-diagnostic.py');pins=m.load_pins()
c=read(O/'source-candidate-13.json');assert hashlib.sha256(json.dumps(c['files'],sort_keys=True,separators=(',',':')).encode()).hexdigest()==c['candidate_sha256'];assert all(sha(p)==h for p,h in c['files'].items())
snap=read(R/'.moriarty-dev/k-macro05-trace106-snapshot.json');draft=read(O/'snapshot-inventory-draft-03.json');different=[k for k in set(snap)|set(draft) if snap.get(k)!=draft.get(k)];assert different==['scope']
req=read(R/'.moriarty-dev/k-macro05-trace106-requisites.json');meta=read(req['observationPath']);assert [(e['path'],e['narHash']) for e in req['entries']]==[(e['path'],e['narHash']) for e in meta['entries']]
kmap=read(R/'.moriarty-dev/k-macro05-trace106-k-package-map.json');actual={str(p) for p in Path(kmap['kRoot']).rglob('*') if p.is_file()};assert actual=={e['path'] for e in kmap['files']};assert all(sha(e['path'])==e['sha256'] for e in kmap['files'])
own=read(R/'.moriarty-dev/k-macro05-trace106-ownership.json');host=m.load_host_evidence(own);ns=m.namespace_uid_for_host(Path('/proc/self/uid_map').read_text(),0,65534)
for path in host:m.verify_ownership(path,host,ns,0)
assert read(own['hostEvidencePath'])['commandSelections']==pins['commandSelections']
sh=next(e for e in pins['selectedExecutables'] if e['role']=='sh');missing=dict(host);del missing[sh['path']]
try:m.verify_protected_path(sh['path'],missing,ns,0)
except m.PreflightError as e:assert e.code=='ARBITRARY_OVERFLOW'
else:raise AssertionError('missing sh ownership accepted')
t=mod('independent_snapshot13',O/'independent-snapshot-regression-12.py');t.m=m;t.base=pins;t.snapshot=snap;t.obs={};buf=io.StringIO();result=unittest.TextTestRunner(stream=buf,verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(t.Tests));assert result.wasSuccessful(),buf.getvalue()
report={'candidate_sha256':c['candidate_sha256'],'candidateFilesVerified':len(c['files']),'snapshotFullParsedDifferenceKeys':different,'snapshotEntriesCompared':len(snap['files']),'requisiteRowsFullyCompared':len(req['entries']),'kPackageEnumeratedAndHashed':len(actual),'hostEntriesRevalidated':len(host),'missingShOwnershipRejected':True,'snapshotTests':{'run':result.testsRun,'failures':len(result.failures),'errors':len(result.errors),'output':buf.getvalue(),'observations':t.obs},'uidMap':Path('/proc/self/uid_map').read_text(),'nativeExecuted':False}
(O/'source-astra-probe-13.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({k:v for k,v in report.items() if k!='snapshotTests'}));print(buf.getvalue())
