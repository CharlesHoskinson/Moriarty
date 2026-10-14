from pathlib import Path
import json,collections,datetime
from graphify.build import build_from_json
from graphify.cluster import cluster,score_all
from graphify.export import to_json
from graphify.analyze import god_nodes,surprising_connections,suggest_questions
from graphify.report import generate
OUT=Path(__file__).resolve().parent/'graphify-out';ROOT=Path('/home/charl/moriarty-utxo-research-2026-10-12')
x=json.loads((OUT/'.graphify_extract.json').read_text());d=json.loads((OUT/'.graphify_detect.json').read_text())
graph=build_from_json(x,root=ROOT,directed=True,dedup=False);print('Graph built',graph.number_of_nodes(),graph.number_of_edges(),flush=True)
communities=cluster(graph);cohesion=score_all(graph,communities)
labels={k:'Community '+str(k) for k in communities}
to_json(graph,communities,str(OUT/'graph.json'),original_links=x['edges'])
gods=god_nodes(graph);surprises=surprising_connections(graph,communities)
report=generate(graph,communities,cohesion,labels,gods,surprises,d,{'input':0,'output':0},str(ROOT))
report+='\n## Model and coverage qualification\n\nLuna requested model: gpt-6-luna. Host usage and returned model identity are not exposed; zero placeholders in native exports are not measured zero cost. See SOURCE-COVERAGE.json and COVERAGE-LUNA-01.json. The full graph includes deterministic source inventory, explicit document heading/citation links, AST and lexical declarations, plus Luna targeted semantic evidence. It is not full semantic review of all29million detected words. Filename exclusions retain metadata. AST warnings/unsupported syntax are retained in graph-build logs.\n'
(OUT/'GRAPH_REPORT.md').write_text(report)
(OUT/'.graphify_analysis.json').write_text(json.dumps({'communities':{str(k):v for k,v in communities.items()},'cohesion':{str(k):v for k,v in cohesion.items()},'gods':gods,'surprises':surprises},ensure_ascii=False))
luna=json.loads((OUT/'.graphify_chunk_01.json').read_text());focus=build_from_json(luna,root=ROOT,directed=True);fc=cluster(focus);to_json(focus,fc,str(OUT/'knowledge-focus.json'))
(OUT/'cost.json').write_text(json.dumps({'requested_model':'gpt-6-luna','actual_returned_model':None,'input_tokens':None,'output_tokens':None,'usd':None,'measurement_status':'host usage not exposed','structural_model_tokens':0},indent=2)+'\n')
(OUT/'GRAPH-VERIFICATION.json').write_text(json.dumps({'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'nodes':graph.number_of_nodes(),'edges':graph.number_of_edges(),'raw_relationships':len(x['edges']),'communities':len(communities),'cohesion':{str(k):v for k,v in cohesion.items()},'source_files':len({str(n.get('source_file')) for n in x['nodes'] if n.get('source_file')}),'semantic_focus_nodes':focus.number_of_nodes(),'semantic_focus_edges':focus.number_of_edges(),'dangling_export_edges':sum(e['source'] not in graph or e['target'] not in graph for e in x['edges'])},indent=2)+'\n');print('Final graph',graph.number_of_nodes(),'nodes',graph.number_of_edges(),'edges',len(communities),'communities',flush=True)
