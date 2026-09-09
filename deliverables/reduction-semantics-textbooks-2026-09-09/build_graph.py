"""Generate a bounded, manually source-reviewed semantic graph; no remote LLM."""
import json
from pathlib import Path
from graphify.build import build_from_json
from graphify.cluster import cluster
from graphify.export import to_html

OUT = Path(__file__).resolve().parent
RAW = '../../raw/sources/reduction-semantics-textbooks-2026-09-09/'
nodes, edges = [], []
def node(id, label, kind, path='', locator=''):
    nodes.append(dict(id=id, label=label, type=kind, source_file=path,
                      locator=locator, evidence_status='research-only'))
def edge(source, target, relation, kind, locator, confidence='high'):
    edges.append(dict(source=source, target=target, relation=relation,
                      evidence_kind=kind, evidence_confidence=confidence,
                      confidence='EXTRACTED' if kind in ('source_fact','repository_observation') else 'INFERRED',
                      source_file=next((n['source_file'] for n in nodes if n['id']==source and n['source_file']), 'BEST-PRACTICES.md'),
                      confidence_score=1.0 if confidence=='high' else .8,
                      extraction_method='EXTRACTED' if kind in ('source_fact','repository_observation') else 'INFERRED',
                      source_locator=locator))

node('plfa','PLFA — CC BY 4.0','textbook',RAW+'plfa-source.tar.gz','Properties; Lambda; extra/EvalContexts (draft)')
node('sf','Software Foundations PLF 7.1 — MIT','textbook',RAW+'sf-plf-complete.tgz','Smallstep; Types; StlcProp')
node('redex','Redex manual 9.0 — Apache-2.0 OR MIT','manual',RAW+'redex-9.0-complete.pdf','Amb Tutorial; Long Tutorial; Reduction Relations')
node('fh1992','Felleisen–Hieb 1992 — historical paper','paper','../../raw/sources/felleisen-hieb-2026-09-09/felleisen-hieb-1992.pdf','§2 Definitions 2.1, 2.3; §3.1')
concepts = {
 'contexts':'Evaluation contexts select the next redex',
 'closure':'Primitive contraction versus context closure',
 'abort':'Failure discards the evaluation context',
 'answers':'Terminal answer versus stuck term',
 'domain':'Admitted and reachable claim domain',
 'determinism':'At most one successor',
 'termination':'Finite control versus fuel and action budget',
 'state':'Explicit state and final observations',
 'binding':'Hole filling versus variable substitution',
 'testing':'One-step, final-result and falsification tests'}
for id,label in concepts.items(): node(id,label,'concept')
source_links=[
 ('redex','contexts','ref/reduction-relations.scrbl:266-313'),
 ('redex','closure','ref/reduction-relations.scrbl:214-313'),
 ('redex','abort','long-tut/wed-mor.scrbl:171-249'),
 ('redex','state','long-tut/wed-mor.scrbl:85-165'),
 ('redex','binding','tut.scrbl:639-678'),
 ('redex','testing','tut.scrbl:804-952'),
 ('plfa','contexts','extra/EvalContexts/Lambda.lagda.md:555-671 (extra draft)'),
 ('plfa','answers','src/plfa/part2/Properties.lagda.md:1324-1395'),
 ('plfa','domain','src/plfa/part2/Properties.lagda.md:36-84'),
 ('plfa','determinism','src/plfa/part2/Properties.lagda.md:1397-1440'),
 ('plfa','termination','src/plfa/part2/Properties.lagda.md:68-75,848-972'),
 ('plfa','binding','src/plfa/part2/Properties.lagda.md:547-768'),
 ('sf','state','Smallstep.v:1342-1487'),
 ('sf','determinism','Smallstep.v:263-295'),
 ('sf','answers','Smallstep.v:Values; Normal Forms; Stuck Terms'),
 ('fh1992','closure','§2 Definitions 2.1 and 2.3'),
 ('fh1992','abort','§3.1 whole-program control reductions')]
for s,t,loc in source_links:
    edge(s,t,'explains','source_fact',loc)
    if s == 'redex':
        edges[-1]['source_file'] = RAW + 'selected/redex/redex-doc/redex/scribblings/' + loc.split(':', 1)[0]

checks={
 'check-order':('First false check wins; no suffix step',['contexts','closure','abort']),
 'check-answers':('Reachable non-answer steps; answers do not',['answers','domain','determinism']),
 'check-bound':('Inspect 24-step abstract trace bound',['termination']),
 'check-state':('Compare full proposal; reject exposes no state/effects',['state','abort']),
 'check-binding':('Define scope only when successor binding executes',['binding']),
 'check-relations':('Compare all one-step successors and full final results',['testing','determinism'])}
for id,(label,cs) in checks.items():
    node(id,label,'proposed_check','BEST-PRACTICES.md','specified-only')
    for c in cs: edge(c,id,'motivates','recommendation','BEST-PRACTICES.md')

files={
 'readme':('README control presentation','../../README.md','Small-step semantics (implemented repayment subset)'),
 'k':('Bounded repayment K rules','../../experiments/moriarty-language/formal/k/moriarty.k','lines 52-83'),
 'codec':('Codec boundary','../../experiments/moriarty-language/formal/k/README.md','Codec boundary'),
 'cases':('16-case finite comparison','../bounded-k-2026-09-09/acceptance.json','complete success/rejection comparisons'),
 'successor':('Successor semantics remain open','../../experiments/moriarty-language/spec/successor/README.md','syntax-only profile'),
 'roadmap':('Formal correspondence remains open','../../ROADMAP.md','MC01/MC04')}
for id,(label,path,loc) in files.items(): node(id,label,'repository_file',path,loc)
for s,t in [('check-order','readme'),('check-order','k'),('check-answers','readme'),('check-bound','readme'),('check-state','codec'),('check-state','cases'),('check-binding','successor'),('check-relations','k'),('check-relations','roadmap')]:
    edge(s,t,'would_check','recommendation','BEST-PRACTICES.md; '+files[t][2])
edge('readme','abort','already_specifies','repository_observation','README ABORT; candidate digest in BEST-PRACTICES.md')
edge('readme','contexts','already_specifies','repository_observation','README E grammar and flat-list equations')
edge('check-bound','termination','is_scoped_inspection_argument','inference','21 K ensure instructions at moriarty.k:60-80; abstract START/EXPAND/PREPARE')
edge('cases','roadmap','does_not_close','open_question','finite comparison is not a correspondence theorem')

graph=dict(schema='moriarty.research-concept-graph/1', directed=True,
           coverage='Three complete source downloads; graph covers only inspected chapters and scoped Moriarty implications.',
           claim_status='S2 recommendations; no proof or new execution evidence', nodes=nodes, edges=edges)
(OUT/'graph.json').write_text(json.dumps(graph,indent=2,ensure_ascii=False)+'\n')
G=build_from_json(graph,directed=True,root=OUT)
communities=cluster(G)
labels={i:' / '.join(G.nodes[n].get('label',n) for n in members if n in ('plfa','sf','redex','readme')) or 'Semantic obligations' for i,members in communities.items()}
to_html(G,communities,str(OUT/'graph.html'),community_labels=labels)
(OUT/'graph-analysis.json').write_text(json.dumps(dict(nodes=len(nodes),edges=len(edges),communities=communities,method='Host-authored source-located edges; graphify directed build and community detection; no external extraction service.'),indent=2)+'\n')
lines=['# Textbook concept map','', 'This is a scoped reading map, not a proof graph. Every proposed check remains specified-only. Complete downloads are described in [CORPUS.md](CORPUS.md). The interactive [graph](graph.html) and typed [JSON](graph.json) preserve source locators and evidence kinds.','', '```mermaid','flowchart LR']
for n in nodes: lines.append(f'  {n["id"].replace("-","_")}["{n["label"].replace(chr(34),chr(39))}"]')
for e in edges: lines.append(f'  {e["source"].replace("-","_")} -->|{e["relation"]}| {e["target"].replace("-","_")}')
lines += ['```','',f'{len(nodes)} nodes; {len(edges)} directed edges. Node types separate textbooks/manual/paper, concepts, proposed checks and repository files. Edge kinds separate source facts, observations, inferences, recommendations and open questions.','', 'No whole-corpus semantic extraction, source compilation, theorem transfer or live runtime validation is claimed.']
(OUT/'GRAPH.md').write_text('\n'.join(lines)+'\n')
print(f'Graph generated: {len(nodes)} nodes, {len(edges)} edges, {len(communities)} communities')
