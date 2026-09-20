import json, collections
from pathlib import Path
from graphify.build import build_from_json
from graphify.cluster import cluster, score_all
from graphify.analyze import god_nodes, surprising_connections, suggest_questions
from graphify.report import generate
from graphify.export import to_json
from graphify.diagnostics import diagnose_extraction, format_diagnostic_report
from graphify.cache import save_semantic_cache
from graphify.detect import save_manifest
from graphify.cli import _stamped_manifest_files
p=Path('/home/charl/.graphify/repos/alcides/aeon');o=p/'graphify-out'
d=json.loads((o/'.graphify_detect.json').read_text());x=json.loads((o/'.graphify_ast.json').read_text())
sem={'nodes':[], 'edges':[], 'hyperedges':[]}
for i in (1,2):
 q=json.loads((o/f'.graphify_chunk_{i:02}.json').read_text())
 for k in sem: sem[k].extend(q.get(k,[]))
save_semantic_cache(sem['nodes'],sem['edges'],sem['hyperedges'],root=str(p),allowed_source_files=d['files']['document'],prompt_file='/home/charl/.agents/skills/graphify/references/extraction-spec.md')
seen={n['id'] for n in x['nodes']}
for n in sem['nodes']:
 if n['id'] not in seen: x['nodes'].append(n);seen.add(n['id'])
x['edges']+=sem['edges'];x['hyperedges']=sem['hyperedges'];x['input_tokens']=0;x['output_tokens']=0
(o/'.graphify_extract.json').write_text(json.dumps(x))
g=build_from_json(x,root=str(p),directed=True)
assert g.number_of_nodes()
c=cluster(g);co=score_all(g,c);gods=god_nodes(g);sur=surprising_connections(g,c)
summary=[];labels={}
for cid,ns in c.items():
 files=collections.Counter(str(g.nodes[n].get('source_file','')) for n in ns)
 leaders=sorted(ns,key=lambda n:g.degree(n),reverse=True)[:5]
 main=files.most_common(1)[0][0];stem=Path(main).stem.replace('_',' ').replace('-',' ')
 labels[cid]=' '.join(stem.split()[:4]).title() or 'Language Components'
 summary.append({'community':cid,'label':labels[cid],'size':len(ns),'files':files.most_common(3),'leaders':[g.nodes[n].get('label',n) for n in leaders]})
(o/'community-inventory.json').write_text(json.dumps(summary,indent=2))
questions=suggest_questions(g,c,labels)
assert to_json(g,c,str(o/'graph.json'),community_labels=labels)
report=generate(g,c,co,labels,gods,sur,d,{'input':0,'output':0},str(p),suggested_questions=questions)
report+='\n## Coverage and accounting qualifications\n\nDirected structural Python graph plus semantic extraction of 40 documents. Excludes 49 image assets. Graphify does not parse .ae and .lark files; selected examples were inspected separately in the study. Agent token usage is unavailable from the host; numeric zero fields are placeholders, not zero-cost claims. Graph edges are navigation evidence, not proof of runtime behavior.\n'
(o/'GRAPH_REPORT.md').write_text(report);(o/'.graphify_labels.json').write_text(json.dumps(labels))
diag=diagnose_extraction(x,directed=True,root=str(p));(o/'health.json').write_text(json.dumps(diag,indent=2));(o/'health.txt').write_text(format_diagnostic_report(diag));print(format_diagnostic_report(diag))
files=_stamped_manifest_files(d['files'],x,p);save_manifest(files,root=str(p),scan_corpus={f for fs in d['files'].values() for f in fs})
(o/'cost.json').write_text(json.dumps({'input_tokens':None,'output_tokens':None,'note':'Host semantic-agent token usage unavailable; AST requires no LLM.'},indent=2))
print(json.dumps({'nodes':len(g),'edges':g.number_of_edges(),'communities':len(c),'gods':gods[:8]},indent=2))
