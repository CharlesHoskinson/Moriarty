from pathlib import Path
import json,collections
import networkx as nx
from graphify.exporters.html import to_html
out=Path(__file__).parent/'graphify-out';x=json.loads((out/'graph.json').read_text());G=nx.DiGraph();communities=collections.defaultdict(list)
for n in x['nodes']:
 d=dict(n);i=d.pop('id');G.add_node(i,**d);communities[d.get('community',6163)].append(i)
for e in x['links']:
 d=dict(e);a=d.pop('source');b=d.pop('target');G.add_edge(a,b,**d)
to_html(G,dict(communities),str(out/'graph.html'),node_limit=7000)
print('Interactive community view covers',G.number_of_nodes(),'exact identity nodes; underlying multirelation graph preserved in graph.json')
