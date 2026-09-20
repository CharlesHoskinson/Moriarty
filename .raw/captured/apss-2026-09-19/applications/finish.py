from pathlib import Path
import json,hashlib,re
R=Path(__file__).parent
M=json.loads((R/'manifest.json').read_text())
visual={x['source_id']:x for x in json.loads((R/'visual-reading.json').read_text())}
authors={'APP01':'Ankit Chiplunkar and Stephane Gosselin / Frontier Research','APP02':'Simon Peyton Jones, Jean-Marc Eber and Julian Seward','APP03':'Pablo Lamela Seijas, Alexander Nemish, David Smith and Simon Thompson','APP04':'Andrei Arusoaie','APP05':'Christian Bräm, Marco Eilers, Peter Müller, Robin Sierra and Alexander J. Summers','APP06':'Sean Bowe, Alessandro Chiesa, Matthew Green, Ian Miers, Pratyush Mishra and Howard Wu','APP07':'Anoma specification contributors','APP08':'Yulia Khalniyazova / Anoma','APP09':'CoW Protocol repository contributors','APP10':'CoW Protocol repository contributors','APP11':'Jeroen Offerijns, Alina Sinelnikova, Vikram Arun, Joey Santoro, Farhaan Ali and João Martins','APP12':'Francisco Giordano, Mark Toda, Matt Rice, Nick Pai, Alexander Lindgren, Mark Gretzke and Chris Cashwell (captured draft)'}
use={'APP01':('Architecture essay','Application versus authority/solver/settlement boundaries'),'APP02':('Primary research paper','Composition beyond a fixed financial-product catalog'),'APP03':('Primary research paper','Language-wide semantic invariants versus application properties'),'APP04':('Primary research paper','Counterexample: compositional syntax does not assure a fair bargain'),'APP05':('Primary research paper','Resource effects and modular collaboration specifications'),'APP06':('Primary research paper','Private assets and exchanges; confidentiality versus anonymity'),'APP07':('Official technical specification','Shared versus application-specific resource predicates'),'APP08':('Primary author explanation','Exact desired resources versus predicates defining acceptable outcomes'),'APP09':('Official implementation documentation','Conditional orders, optional discovery, cumulative funding risk'),'APP10':('Official implementation documentation','Hard pool validity versus advisory order generation'),'APP11':('Normative interface standard','Asynchronous request/claim semantics'),'APP12':('Draft interface standard','Solver-facing interoperability versus settlement verification')}
for m in M:
 m['authors_or_publisher']=authors[m['id']]
 m['pdf_pages_visually_read']=visual.get(m['id'],{}).get('pages_visual_read_one_based',[])
 if m['id']=='APP07':m['version']='Captured page reports Anoma Specification v0.1.2-bcd7b10c72; mutable main URL'
 if m['id']=='APP11':m['version']='Final, captured 2026-09-19'
 if m['id']=='APP12':m['version']='Draft resolver-based specification, captured 2026-09-19'
 m['coverage']='Selected relevant sections only; no full implementation audit'
(R/'manifest.json').write_text(json.dumps(M,indent=2)+'\n')
text='''---
id: apss.applications.reference
title: Applications literature reference
status: draft
documentation_type: reference
---

# Applications literature reference

Bounded first pass: twelve substantive sources across ten independence groups. Anoma's two sources and CoW's two sources are related, not independent corroborations. Dates below distinguish publication from capture; historical papers do not establish current deployment properties. All full captures, SHA-256 values, final URLs and source independence keys are in [manifest.json](manifest.json). The source notes separate claims, inference and limitations.

| ID | Source and kind | Application relevance | Detailed annotation |
|---|---|---|---|
'''
for m in M:
 sid=m['id'];kind,why=use[sid]
 text+=f"| {sid} | [{m['title']}]({m.get('pinned_url',m['final_url'])}) — {kind} | {why} | [Source note](notes/{sid}.md) |\n"
text+='\n## Bibliographic records and inspected evidence\n\n'
for m in M:
 sid=m['id'];pages=m['pdf_pages_visually_read']
 text+=f"**{sid}. {m['title']}** — {authors[sid]}. {m.get('version') or m['publication_date'] or 'Publication date not determined'}. Retrieved {m['retrieved_at']}. "
 text+=f"PDF pages visually inspected: {', '.join(map(str,pages))}. " if pages else 'HTML/Markdown sections inspected; no PDF page claim. '
 if m.get('commit'):text+=f"Repository commit `{m['commit']}`; pinned bytes match the initial capture. "
 text+=f"Capture SHA-256 `{m['sha256']}`.\n\n"
text+='''## Coverage boundaries

The five PDFs total 157 rendered pages; twelve selected pages were actually visually read. PixelRAG `pixelshot` rendered all pages at 100 DPI. Rendering is not reading. [visual-reading.json](visual-reading.json) names exact one-based PDF pages and zero-based tile files. Additional text extraction supported metadata and section navigation; it is not claimed as full-page visual inspection.

No benchmarks, proof artifacts, full code audit, contract deployments, live trading or network feasibility experiments were reproduced. CAKE and Anoma explanatory claims are not promoted into theorems. CoW documentation was pinned to the exact latest commit affecting each captured file and independently compared to the initial raw bytes. The current ERC-7683 draft is not interchangeable with older versions.

## Evidence classes

| Class | What it establishes here | What it does not establish |
|---|---|---|
| Captured source statement | What named authors/specifications say in these bytes | Truth of all economic or security claims |
| Selected visual reading | Actual content on listed paper pages | Reading of unlisted pages |
| Design inference | A motivated Moriarty design option or proposed requirement | Implemented feature or accepted proof |
| Conceptual exercise | Arithmetic consequences of stated assumptions | Execution, formal proof or ledger settlement |

The documentation split follows [Diátaxis](https://www.diataxis.fr/): [explanation](explanation.md) develops the design reasoning; [how-to](how-to.md) supports a concrete design audit; [tutorial](tutorial.md) teaches a bounded conceptual exercise. This reference is for lookup, not a deployment procedure.
'''
(R/'reference.md').write_text(text)
claims=[]
for m in M:
 sid=m['id']; note=(R/'notes'/f'{sid}.md').read_text()
 claim=note.split('**Source claims.** ')[1].split('\n\n')[0]
 claims.append({'id':f'APPS-CLAIM-{sid[-2:]}','source_id':sid,'text':claim,'assessment':'provisional','claim_kind':'source_statement','source_sha256':m['sha256'],'location':f'notes/{sid}.md','scope':'What the captured source states; no independent implementation reproduction','independence_key':m['independence_key']})
(R/'claims.json').write_text(json.dumps(claims,indent=2)+'\n')
# Independent arithmetic checks for the conceptual exercise, not product tests.
assert 1099*1820==2000180
assert 99+1==100 and 110+1==111 and 110-11+1==100
assert len(M)==12 and len({x['independence_key'] for x in M})==10
for m in M:
 assert hashlib.sha256((R/m['path']).read_bytes()).hexdigest()==m['sha256']
for entry in visual.values():
 for p in entry['tiles']:assert (R/p).is_file()
print('12 source hashes verified; 12 visual page locators verified; tutorial arithmetic verified')
