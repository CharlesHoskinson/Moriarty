import pathlib,json,os,hashlib
P=pathlib.Path('/tmp/moriarty-production-persistence-r1-probes')
rows=json.loads((P/'results.json').read_text())+json.loads((P/'write-results.json').read_text())
checks=[]
for r in rows:
 name=r['name']
 if 'result' not in r:
  f=r['facts'];u=f['unit']
  if name=='original_showTimeout':assert u['contained'] and not r['injectionCalls']
  else:
   assert not u['contained'] and not f['complete'] and r['injectionCalls']
   assert any(x['resource']=='unit:fixture.service' for x in f['outstandingOwners'])
   if name!='after_kill_timeout':assert u['disposition']=='observation-unavailable'
  checks.append(name);continue
 d=r['result']
 if name=='baseline' or name.startswith('missed_'):
  assert r['exitCode']==0 and d['status']=='PROCESS_SUCCESS'
  if name.startswith('missed_'):
   target=name.split('_',2)[2]
   assert target not in [c[0] for c in r['writerCalls']]
 else:
  assert r['exitCode']==3 and d['status']=='PROCESS_UNKNOWN' and len(r['faultHits'])==1
  hit=json.loads(r['faultHits'][0]);assert hit['fdPath'].startswith('/tmp/moriarty-loan-executor-')
  try:os.kill(hit['pid'],0)
  except ProcessLookupError:pass
  else:raise AssertionError('worker still exists: '+str(hit['pid']))
  assert d['containmentComplete'] and d['financialAcceptance']=='pending' and not d['retryAllowed']
  stops=[i for i,c in enumerate(r['commands']) if 'stop' in c and 'moriarty-loan-fixture.service' in c]
  kills=[i for i,c in enumerate(r['commands']) if 'kill' in c]
  assert len(stops)==1
  if name.startswith('terminal-observation'):
   assert d['failureCode']=='EVIDENCE_WRITE_FAILED' and not d['terminalEvidencePersisted'] and not d['stopReceiptPersisted'] and d['stopReturnCode'] is None
   assert len(kills)==2 and min(kills)<stops[0]
   assert 'terminal-observation' not in [e['kind'] for e in d['evidence']]
  else:
   assert d['failureCode']=='STOP_RECEIPT_FAILED' and d['terminalEvidencePersisted'] and not d['stopReceiptPersisted'] and d['stopReturnCode']==0
   assert d['rawMainExit']=={'kind':'exit','code':0}
  if name.endswith('stall'):assert r['elapsed']<3
 checks.append(name)
m=json.load(open('/tmp/moriarty-production-round1-evidence/source-manifest.json'))
for rel,sha in m['files'].items():
 for root in ('/tmp/moriarty-production-round1-freeze','/tmp/moriarty-production-persistence-diagnosis-r1-work'):
  assert hashlib.sha256((pathlib.Path(root)/rel).read_bytes()).hexdigest()==sha
print(json.dumps({'caseCount':len(checks),'cases':checks,'allAssertionsPassed':True,'frozenFilesVerified':len(m['files'])},indent=2))
