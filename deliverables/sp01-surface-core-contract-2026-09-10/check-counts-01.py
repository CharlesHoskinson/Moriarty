"""Counts handwritten expected node-entry sequences, not an evaluator."""
import pathlib,json
v=json.loads((pathlib.Path(__file__).parent/'cases-01.json').read_text());assert len(v['cases'])==14
for c in v['cases']:assert c['workUsed']==len(c['enteredNodes']),c['id']
print(json.dumps({'status':'PASS','specifiedNodeEntryCounts':14,'runtimeExecuted':False}))
