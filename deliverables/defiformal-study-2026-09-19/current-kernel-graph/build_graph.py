from pathlib import Path
import json,re,hashlib,collections,networkx as nx
from graphify.export import to_json,to_html
ROOT=Path('/home/charl/research/defiformal-moriarty-scope-2026-09-19/repo')
OUT=ROOT.parent/'current-kernel-graph'
PIN='33b9a9550ac1ed12c83c32d15277e530741787db'
files=sorted([ROOT/'lean/DefiKernel.lean',*(ROOT/'lean/DefiKernel').rglob('*.lean')]);assert len(files)<500
# Comment masking preserves offsets and lines, including nested Lean block comments.
def mask_comments(text):
 out=list(text);i=0;depth=0;string=False
 while i<len(text):
  if depth:
   if text.startswith('/-',i):out[i:i+2]='  ';depth+=1;i+=2;continue
   if text.startswith('-/',i):out[i:i+2]='  ';depth-=1;i+=2;continue
   if text[i]!='\n':out[i]=' '
   i+=1;continue
  if string:
   if text[i]=='\\':i+=2;continue
   if text[i]=='"':string=False
   i+=1;continue
  if text[i]=='"':string=True;i+=1;continue
  if text.startswith('/-',i):depth=1;out[i:i+2]='  ';i+=2;continue
  if text.startswith('--',i):
   while i<len(text) and text[i]!='\n':out[i]=' ';i+=1
   continue
  i+=1
 return ''.join(out)
def module(p):return str(p.relative_to(ROOT/'lean').with_suffix('')).replace('/','.')
def family(m):return m.split('.')[1] if len(m.split('.'))>2 else 'Root'
modules={module(p):p for p in files};G=nx.DiGraph();manifest=[];edges=[]
for m,p in modules.items():
 rel=str(p.relative_to(ROOT));digest=hashlib.sha256(p.read_bytes()).hexdigest();fam=family(m)
 manifest.append({'module':m,'path':rel,'sha256':digest,'bytes':p.stat().st_size,'family':fam})
 G.add_node(m,label=m.removeprefix('DefiKernel.'),file_type='code',source_file=rel,source_location=f'{rel}:1',family=fam,sha256=digest,external=False)
for m,p in modules.items():
 text=p.read_text();clean=mask_comments(text)
 for match in re.finditer(r'^[ \t]*(?:public[ \t]+)?import[ \t]+([^\n]+)',clean,re.M):
  line=clean.count('\n',0,match.start())+1
  for imp in match.group(1).split():
   if not re.fullmatch(r'[A-Za-z_][A-Za-z_0-9\.]*(?:\.[A-Za-z_0-9]+)*',imp):raise ValueError((m,imp))
   if imp not in G:G.add_node(imp,label=imp,file_type='external',family='External',external=True)
   row={'source':m,'target':imp,'relation':'textual_import','confidence':'EXTRACTED','source_file':str(p.relative_to(ROOT)),'source_location':f'{p.relative_to(ROOT)}:{line}','line':line,'target_in_scope':imp in modules}
   edges.append(row);G.add_edge(m,imp,**{k:v for k,v in row.items() if k not in ('source','target')})
families=collections.defaultdict(list)
for m in G:families[G.nodes[m]['family']].append(m)
communities={i:sorted(ms) for i,(_,ms) in enumerate(sorted(families.items()))};labels={i:f for i,f in enumerate(sorted(families))}
assert to_json(G,communities,str(OUT/'graph.json'),community_labels=labels)
assert to_html(G,communities,str(OUT/'graph.html'),community_labels=labels)
closure=nx.descendants(G,'DefiKernel') & set(modules)
familydeps=collections.Counter((family(x['source']),family(x['target'])) for x in edges if x['target_in_scope'] and family(x['source'])!=family(x['target']))
summary={'commit':PIN,'source_files':len(files),'nodes':len(G),'edges':G.number_of_edges(),'import_occurrences':len(edges),'external_modules':sorted(set(G)-set(modules)),'family_module_counts':dict(collections.Counter(family(m) for m in modules)),'entrypoint_import_closure_count':len(closure),'entrypoint_not_reachable':sorted(set(modules)-closure-{'DefiKernel'}),'family_dependencies':[{'source':a,'target':b,'import_count':n} for (a,b),n in sorted(familydeps.items())],'cyclic_components':[sorted(c) for c in nx.strongly_connected_components(G) if len(c)>1]}
(OUT/'module-hashes.json').write_text(json.dumps(manifest,indent=2));(OUT/'imports.json').write_text(json.dumps(edges,indent=2));(OUT/'summary.json').write_text(json.dumps(summary,indent=2))
limits='''# Current DefiKernel module map\n\nPinned commit: `'''+PIN+'''`. This replaces no historical evidence. It is a bounded textual module/import graph, **not Lean AST extraction, elaborated proof dependencies, theorem validation or source/runtime correspondence**. Import direction is consumer → dependency. Comment masking handles nested block comments; declaration and proof bodies are not analyzed. External imports are terminal placeholder nodes. Families are directory groups, not inferred proof domains. Root reachability does not establish that a file is dead, accepted, built, checked, or suitable for deployment. No Lean builds, remote models, or theorem/axiom checks ran. No tracked source or tracked graph changed. Extraction uses zero LLM tokens; host analysis tokens are unmeasured.\n\n'''
lines=[limits,f"{len(files)} source modules; {len(G)} nodes; {G.number_of_edges()} distinct directed import edges; {len(edges)} import occurrences. Root entrypoint reaches {len(closure)} selected modules.\n",'## Families\n','| Family | Modules |\n|---|---:|']
lines += [f'| {f} | {n} |' for f,n in sorted(summary['family_module_counts'].items())]
lines += ['\n## Cross-family imports\n','| Importer | Dependency | Imports |\n|---|---|---:|']
lines += [f'| {a} | {b} | {n} |' for (a,b),n in sorted(familydeps.items())]
lines += ['\n## Entry-point scope\n', 'Modules absent from the root import closure (presence here is not a defect or dead-code claim):\n']
lines += [f'- `{m}`' for m in summary['entrypoint_not_reachable']]
(OUT/'GRAPH_REPORT.md').write_text('\n'.join(lines)+'\n')
print(json.dumps(summary,indent=2))
