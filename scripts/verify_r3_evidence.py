from pathlib import Path
import json,hashlib,csv,re,subprocess,datetime
p=Path(__file__).resolve().parents[1]
h=lambda b:hashlib.sha256(b).hexdigest()
m=json.loads((p/'evidence/midnight-docs-2026-09-07/coverage-manifest.json').read_text())
assert len(m['receipts'])==1289
for r in m['receipts']:
 b=(p/r['local_markdown_path']).read_bytes()
 assert h(b)==r['sha256'] and len(b)==r['byte_count'] and r['http_status']==200
 assert (p/r['local_text_path']).read_bytes()==b.decode('utf-8',errors='replace').encode()
for r in m['discovery'].values():
 assert h((p/r['local_path']).read_bytes())==r['sha256']
rows=list(csv.DictReader((p/'evidence/source-inventory.csv').open()))
assert len({r['source_id'] for r in rows})==len(rows)
for r in rows:
 if int(r['source_id'].split('-')[1])>=51: assert h((p/r['local_path']).read_bytes())==r['sha256'],r['source_id']
s=json.loads((p/'evidence/moriarty-native-ivc-r3-2026-09-07/summary.json').read_text())
assert s['nativeProofsProduced']==0 and s['fixedK']==17
run_results=[json.loads((p/f'evidence/moriarty-native-ivc-r3-2026-09-07/attempt-0{i}/result.json').read_text()) for i in range(1,4)]
assert all(r['exitCode']==101 for r in run_results)
assert abs(sum(r['elapsedSeconds'] for r in run_results)-s['totalActiveSeconds'])<1e-8
log=(p/'evidence/moriarty-native-ivc-r3-2026-09-07/attempt-03/native.stdout.txt').read_text()
assert s['cause'] in log
assert log.count('"stage":"local_circuit"')==4
for f in ['experiments/moriarty-native-ivc-r3/run-native.py','experiments/moriarty-native-ivc-r3/scripts/midnight_docs_acquire.py']:
 compile((p/f).read_text(),f,'exec')
changed=['AGENTS.md','docs/research/2026-09-06-intents-report-integration.md','experiments/moriarty-native-ivc-r3/README.md','wiki/formal-assurance.md','wiki/index.md','wiki/log.md','wiki/midnight-repositories.md','wiki/research-journal.md']
new=['docs/superpowers/plans/2026-09-07-r3-native-and-network.md']
links=0
for filename in changed+new:
 if not filename.endswith('.md'):continue
 f=p/filename
 for target in re.findall(r'\]\(([^\s)]+)\)',f.read_text()):
  if '://' in target or target.startswith(('#','/','mailto:')):continue
  path=target.split('#')[0]
  if not path:continue
  assert (f.parent/path).exists(),(filename,target)
  links+=1
report={'checkedUtc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'documentationPageHashes':1289,'discoveryHashes':len(m['discovery']),'newSourceHashes':'pass','nativeReceiptConsistency':'pass','nativeRerun':False,'pythonSyntax':'pass','changedMarkdownLocalLinks':links,'result':'PASS'}
print(json.dumps(report))
