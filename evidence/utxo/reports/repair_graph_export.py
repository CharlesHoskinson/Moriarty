from pathlib import Path
import json,collections,datetime
out=Path(__file__).parent/'graphify-out'
# Preserve source identities exactly. Graphify canonicalization merged distinct
# inventory records with AST/document twins; this export must not conflate them.
raw=json.loads((out/'.graphify_extract.json').read_text());print('raw loaded',len(raw['nodes']),len(raw['edges']),flush=True)
old=json.loads((out/'graph.json').read_text());com={n['id']:n.get('community') for n in old['nodes']};filecom={n.get('source_file'):n.get('community') for n in old['nodes']};del old
seen=set();links=[]
for e in raw['edges']:
 sig=(e['source'],e['target'],e.get('relation'),e.get('source_file'),e.get('source_location'),e.get('confidence'))
 if sig not in seen:seen.add(sig);links.append(e)
ids={n['id'] for n in raw['nodes']}
assert all(e['source'] in ids and e['target'] in ids for e in links)
for n in raw['nodes']:
 n['community']=com.get(n['id'],filecom.get(n.get('source_file'),6163))
 n['norm_label']=str(n.get('label','')).lower()
(out/'canonicalized-analysis-graph.json').write_bytes((out/'graph.json').read_bytes())
x={'directed':True,'multigraph':True,'graph':{'provenance':'exact retained raw extraction, edge repetitions removed; source identities never merged','built_at':datetime.datetime.now(datetime.timezone.utc).isoformat()},'nodes':raw['nodes'],'links':links,'hyperedges':raw.get('hyperedges',[])}
(out/'graph.json').write_text(json.dumps(x,ensure_ascii=False,separators=(',',':'))+'\n')
print('Exact graph',len(ids),'nodes',len(links),'relationships; removed',len(raw['edges'])-len(links),'identical edge repetitions',flush=True)
