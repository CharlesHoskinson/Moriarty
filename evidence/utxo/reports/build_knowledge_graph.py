#!/usr/bin/env python3
"""Combine full retained-source coverage, AST structure and Luna evidence."""
from pathlib import Path
import hashlib,json,re,collections,datetime,urllib.parse
from lxml import html
ROOT=Path('/home/charl/moriarty-utxo-research-2026-10-12')
OUT=Path(__file__).resolve().parent/'graphify-out'
detect=json.loads((OUT/'.graphify_detect.json').read_text())
ast=json.loads((OUT/'.graphify_ast.json').read_text())
luna=json.loads((OUT/'.graphify_chunk_01.json').read_text())
nodes={n['id']:n for n in ast['nodes']};edges=list(ast['edges']);rows=[]
skip=set(detect.get('skipped_sensitive',[]));urls={};payloads={}
def nid(kind,value):return kind+'_'+hashlib.sha256(value.encode()).hexdigest()[:24]
def node(identifier,label,kind,path=None,**attrs):
 nodes.setdefault(identifier,{'id':identifier,'label':label,'file_type':kind,'source_file':str(path) if path else None,'source_location':None,**attrs});return identifier
def edge(a,b,relation,path=None,**attrs):
 if a!=b:edges.append({'source':a,'target':b,'relation':relation,'confidence':'EXTRACTED','confidence_score':1.0,'weight':1.0,'source_file':str(path) if path else None,'source_location':None,**attrs})
corpus=node('corpus_utxo_eutxo','UTXO/eUTXO research corpus','concept',ROOT)
def folder(p):
 if p==ROOT:return corpus
 identifier=nid('directory',str(p.relative_to(ROOT)));node(identifier,str(p.relative_to(ROOT)),'concept',p,coverage_layer='directory');edge(folder(p.parent),identifier,'contains',p);return identifier
for receipt in list(ROOT.glob('*/*.receipt.json'))+list(ROOT.glob('cardano-*/pages/*.receipt.json')):
 d=json.loads(receipt.read_text());p=d.get('payload_path')
 if p:payloads[str(Path(p))]=d
 u=d.get('requested_url')
 if u and p:urls[u.rstrip('/')]=str(Path(p))
files=[]
for p in ROOT.rglob('*'):
 if not p.is_file() or p.is_symlink():continue
 rel=p.relative_to(ROOT)
 if any(x in {'.git','.foreman','__pycache__','node_modules','graphify-out'} for x in rel.parts):continue
 files.append(p)
content={};doc_sections=0;lexical=0
for p in sorted(files):
 rel=p.relative_to(ROOT);identifier=nid('file',str(rel));sensitive=str(p) in skip
 info=payloads.get(str(p),{});kind='paper' if p.suffix=='.pdf' else 'code' if str(p) in detect['files'].get('code',[]) or p.suffix in {'.hs','.lhs','.ak','.agda','.lean','.qnt','.compact','.mori'} else 'image' if p.suffix in {'.png','.jpg','.jpeg','.gif','.svg','.webp'} else 'document'
 data=p.read_bytes();digest=hashlib.sha256(data).hexdigest()
 node(identifier,info.get('title') or str(rel),kind,p,coverage_layer='source_inventory',sha256=digest,bytes=len(data),source_url=info.get('requested_url'),acquisition_status=info.get('status'),body_indexing='excluded_by_filename_policy' if sensitive else 'structural',semantic_review='luna_targeted' if str(p) in {n.get('source_file') for n in luna['nodes']} else 'not_model_reviewed')
 edge(folder(p.parent),identifier,'contains',p)
 if digest in content:edge(identifier,content[digest],'identical_retained_bytes',p)
 else:content[digest]=identifier
 rows.append({'path':str(p),'id':identifier,'sha256':digest,'sensitive_filename_exclusion':sensitive,'bytes':len(data)})
 if sensitive or len(data)>12_000_000:continue
 headings=[];links=[];text=None
 if p.suffix in {'.md','.mdx','.rst'}:
  text=data.decode('utf-8','replace');headings=[(i,l.lstrip('# ').strip()) for i,l in enumerate(text.splitlines(),1) if re.match(r'^#{1,6} ',l)];links=re.findall(r'\[[^\]]*\]\((https?://[^)\s]+)\)',text)
 elif p.suffix=='.html':
  try:
   page=html.fromstring(data);parts=page.xpath('//article|//main|//*[@id="content"]')
   if parts:
    for part in parts:
     headings.extend((None,e.text_content().strip()) for e in part.xpath('.//h1|.//h2|.//h3|.//h4'))
     links.extend(part.xpath('.//a/@href'))
  except Exception:pass
 for line,label in headings:
  if not label or len(label)>220:continue
  section=node(nid('section',str(rel)+'\0'+label),label,'concept',p,coverage_layer='explicit_document_heading',source_location=f'L{line}' if line else None)
  edge(identifier,section,'has_section',p);doc_sections+=1
 for raw in links:
  base=info.get('canonical_url') or info.get('requested_url') or ''
  url=urllib.parse.urljoin(base,raw);parts=urllib.parse.urlsplit(url)
  if parts.scheme not in {'https','http'}:continue
  url=urllib.parse.urlunsplit((parts.scheme,parts.netloc,parts.path,'',''))
  target=node(nid('url',url),url,'document',p,source_url=url,coverage_layer='explicit_reference')
  edge(identifier,target,'references',p)
  retained=urls.get(url.rstrip('/'))
  if retained:edge(target,nid('file',str(Path(retained).relative_to(ROOT))),'retained_as',p)
 if p.suffix in {'.hs','.lhs','.ak','.agda','.lean','.qnt','.compact','.mori'}:
  text=data.decode('utf-8','replace')
  patterns={'.hs':r'^(?:module|data|newtype|type|class)\s+([A-Za-z_][A-Za-z0-9_\.]*)|^([a-z_][A-Za-z0-9_\']*)\s*::', '.lhs':r'^(?:>\s*)?(?:module|data|newtype|type|class)\s+([A-Za-z_][A-Za-z0-9_\.]*)', '.ak':r'^(?:pub\s+)?(?:fn|type|validator|const)\s+([A-Za-z_][A-Za-z0-9_]*)', '.agda':r'^(?:module|record|data)\s+([^\s:({]+)', '.lean':r'^(?:private\s+|protected\s+)?(?:def|theorem|lemma|structure|inductive|class|namespace)\s+([^\s:({]+)', '.qnt':r'^\s*(?:module|action|pure\s+def|def|val|var|type)\s+([A-Za-z_][A-Za-z0-9_]*)', '.compact':r'^(?:export\s+)?(?:ledger|circuit|struct|enum|witness)\s+([A-Za-z_][A-Za-z0-9_]*)', '.mori':r'^\s*(?:agreement|action|record|operation|state|asset|unit)\s+([A-Za-z_][A-Za-z0-9_]*)'}
  for line,value in enumerate(text.splitlines(),1):
   match=re.search(patterns[p.suffix],value)
   if match:
    label=next(x for x in match.groups() if x);symbol=node(nid('lexical',str(rel)+'\0'+label),label,'code',p,source_location=f'L{line}',coverage_layer='lexical_declaration_not_AST',verification='syntax_occurrence_only');edge(identifier,symbol,'declares',p);lexical+=1
# Every actual AST symbol links back to its retained source, regardless of data-only files.
for n in list(ast['nodes']):
 source=n.get('source_file')
 if not source:continue
 p=Path(source);p=p if p.is_absolute() else ROOT/p
 if p.is_relative_to(ROOT):edge(nid('file',str(p.relative_to(ROOT))),n['id'],'defines_AST_symbol',p)
for n in luna['nodes']:
 nodes[n['id']]={**n,'coverage_layer':'luna_semantic_evidence','extractor_requested_model':'gpt-6-luna'}
 p=Path(n['source_file']);edge(nid('file',str(p.relative_to(ROOT))),n['id'],'has_Luna_extracted_concept',p)
edges.extend(luna['edges'])
known=set(nodes);dangling=[e for e in edges if e['source'] not in known or e['target'] not in known]
edges=[e for e in edges if e['source'] in known and e['target'] in known]
coverage={'built_at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'files_in_inventory':len(rows),'supported_files_detected':detect['total_files'],'corpus_word_estimate':detect.get('total_words'),'excluded_body_files':sum(r['sensitive_filename_exclusion'] for r in rows),'document_headings':doc_sections,'lexical_declarations':lexical,'AST_nodes':len(ast['nodes']),'Luna_nodes':len(luna['nodes']),'Luna_reviewed_paths':20,'unresolved_structural_edges':len(dangling),'token_cost':{'input':None,'output':None,'status':'host usage not exposed; structural extraction uses no model'},'limits':['Every retained non-Git source file has provenance representation; structural coverage does not equal full-body semantic review.','Luna reviewed20 evidence files with targeted limits; other explicit heading/citation/code layers are deterministic.','Unsupported syntax, media and filename exclusions retain inventory metadata rather than invented semantics.']}
(OUT/'SOURCE-COVERAGE.json').write_text(json.dumps(coverage,indent=2)+'\n');(OUT/'SOURCE-FILES.json').write_text(json.dumps(rows)+'\n')
(OUT/'.graphify_extract.json').write_text(json.dumps({'nodes':list(nodes.values()),'edges':edges,'hyperedges':luna.get('hyperedges',[]),'input_tokens':0,'output_tokens':0,'token_counts_status':'unknown'},ensure_ascii=False))
print(json.dumps(coverage,indent=2),flush=True)
