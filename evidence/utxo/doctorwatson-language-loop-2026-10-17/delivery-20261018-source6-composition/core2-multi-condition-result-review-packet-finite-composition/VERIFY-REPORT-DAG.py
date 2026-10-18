from pathlib import Path
import json,hashlib
P=Path(__file__).resolve().parent
D=json.loads((P/'REPORT-DAG.json').read_text())
nodes=D['nodes']
def decode(i):
    tag,value=nodes[i]
    if tag=='v':return value
    if tag=='a':return [decode(j) for j in value]
    if tag=='o':return {k:decode(j) for k,j in value}
    raise AssertionError(tag)
checks=json.loads((P/'REPORT-DAG-ROUNDTRIP.json').read_text())
assert hashlib.sha256((P/'REPORT-DAG.json').read_bytes()).hexdigest()==checks['dag_sha256']
for row in checks['all8_raw_files_roundtrip']:
    decoded=decode(row['root'])
    original=(P/row['path']).read_bytes()
    rendered=(json.dumps(decoded,ensure_ascii=True,indent=2)+'\n').encode()
    assert decoded==json.loads(original)
    assert rendered==original
    assert len(original)==row['raw_bytes']
    assert hashlib.sha256(original).hexdigest()==row['sha256']
print('8 complete original report/stdout files round-trip exactly; review metadata only')
