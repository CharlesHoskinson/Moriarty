from pathlib import Path
import csv,io,json,hashlib,re,datetime
R=Path('/home/charl/Moriarty'); D=R/'deliverables/security-token-transformations-2026-09-09'
now=datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ'); day=now[:10]
sha='4ad282c993aeeeca777e659d9ebbdd50a914f69cfd46f123845f162b2868fdab'; raw=f'.raw/captured/{sha}.md'; sid='src-703d9f7d448fae632f90'; legacy='SRC-0110'
assert hashlib.sha256((R/raw).read_bytes()).hexdigest()==sha
H=lambda b:hashlib.sha256(b).hexdigest()
sl=json.loads((R/'wiki/meta/ledgers/source-ledger.json').read_text());cl=json.loads((R/'wiki/meta/ledgers/claim-ledger.json').read_text());ci=json.loads((R/'wiki/meta/legacy-claim-index.json').read_text())
assert legacy not in sl['legacy_source_ids'] and sid not in sl['sources']
for n in range(943,946):assert f'CLM-{n:04d}' not in ci['claims']
pages=['wiki/moriarty-architecture.md','wiki/security.md','wiki/index.md','wiki/log.md','wiki/hot.md']
link='../deliverables/security-token-transformations-2026-09-09'
additions={
 pages[0]:f'''\n\n## Assets, claims and transformations — 2026-09-09

**CLM-0943.** The supplied security-token report separates financial lifecycle and entitlement from identity, transfer control and chain enforcement. Its useful contribution to Moriarty is an explicit account of how wrapping, pledging, liquidation, recovery and redemption change claims and retain obligations. Source: SRC-0110, report lines 317–385 and 539–555; report date 2026-09-09; secondary descriptive synthesis; reviewed {day}; source argument, S2; not reproduced; confidence medium. Its external citations are opaque and were not independently verified.

**CLM-0944.** Recommend bounded asset/claim/encumbrance records and operation-specific transformation rules, with reusable financial and policy profiles. Keep asset quantities distinct from share units, economic exposure, legal title and nominal debt. Normal and exceptional authority must be separate. This extends the existing token-indexed amount/residual-duty direction; it does not add one Core constructor per standard. Source: SRC-0110, lines 317–473; S2 design inference/recommendation, reviewed {day}; not implemented or reproduced; confidence medium. [Three approaches, proposed semantics and eight cases]({link}/DESIGN-IMPLICATIONS.md).

Place the specification in SP01, types/EBNF in SP02, and Felleisen–Hieb reductions/K in SP03. SP07 owns scheduled servicing; SP08 owns DeFi transformations and pending claims; SP06/SP09 bind history and ledger correspondence; SP10 handles private bounded composition; SP11 conformance and SP12 developer release complete the path. SP04 requalifies affected native components. SP05 supplies Docker then Midnight Preview evidence only after the profile is admitted. The [exact sprint crosswalk]({link}/DESIGN-IMPLICATIONS.md#proposed-fit-in-the-existing-agenda) preserves MC/RP gates, existing fixture counts and the shared-file ownership order. These are proposed refinements, not roadmap acceptance or new grammar. See [[wiki/security#Security-token policy paths — 2026-09-09|policy-path risks]].
''',
 pages[1]:f'''\n\n## Security-token policy paths — 2026-09-09

**CLM-0945.** The report proposes policy-path completeness: ordinary transfers and exceptional issuance/burn, recovery, liquidation, wrapper and migration paths must each apply their defined policy and scoped authority. A successful deposit does not establish a safe withdrawal or liquidation path; custody and receipt transfers can change who holds the economic claim. Source: SRC-0110, lines 365–413 and 447–473; report date 2026-09-09; secondary descriptive synthesis; reviewed {day}; S2 source argument and test recommendation; not implemented or reproduced; confidence medium. A cryptographic credential proves its declared predicate under issuer/freshness assumptions, not the truth of external facts or legal compliance.

[Proposed AT01–AT08 cases]({link}/DESIGN-IMPLICATIONS.md#concrete-proposed-cases) cover wrapper restrictions, partial encumbrance/liquidation, pending redemption, scoped recovery, record dates, stale policy/migration, multi-asset batches and private eligibility. Bind asset domains and policy versions, preserve residual obligations, and distinguish a settlement hold from a freeze. Forced recovery is a new authorized transition, not deletion of history. All eight cases remain unexecuted proposals.

The report's CIP-0113 mixed-policy, unfracking and third-party-action concerns remain contested assurance questions. Its current status, audit, deployment and legal assertions have not been independently verified in this intake. Opaque citation markers and two missing sandbox catalogs prevent treating its claimed primary-source review as our own evidence. [Full source and graph]({link}/README.md); [[wiki/moriarty-architecture#Assets, claims and transformations — 2026-09-09|language and agenda fit]].
''',
 pages[2]:f'''\n\n## Security tokens and asset transformations — 2026-09-09

[Report, interactive graph and design analysis]({link}/README.md), SRC-0110, adds asset/claim/encumbrance distinctions and eight proposed transformation cases. CLM-0943–CLM-0945 are maintained in [[wiki/moriarty-architecture|architecture]] and [[wiki/security|security]]. Full report read; 100 graph nodes, 239 directed edges and seven communities. External citations remain unverified. Proposed SP01–SP12 refinements preserve current MC/RP acceptance gates.
''',
 pages[3]:f'''\n\n## [2026-09-09] ingest | Security-token report and asset transformations

Captured the supplied 97,006-byte report unchanged as SRC-0110; all 565 lines read. Capture `capture-security-token-report9-20260909`; ingest `ingest-security-token-report9-20260909`. [Dossier and graph]({link}/README.md) contain 100 nodes, 239 directed edges, seven communities, three extracted hyperedges and eight unexecuted proposed cases. CLM-0943–CLM-0945 extend architecture/security synthesis with explicit claim continuity and policy-path completeness. SRC-0109 is reserved by the existing syntax research branch; its identity is not reused. External citation tokens and two missing catalogs remain unresolved. No network requests, language implementation, proofs or Midnight transactions were performed by this intake.
''',
 pages[4]:f'''\n\n[Security-token report and asset transformations]({link}/README.md): SRC-0110 recommends explicit assets, claims, encumbrances and operation-specific authority. Define the model in SP01, types and reductions/K in SP02–03, servicing/DeFi behavior in SP07–08, and history/composition/conformance in SP06/SP09–12. Preserve the current admitted SP05 loan/swap work; test each later supported profile on Docker then Preview. The eight new cases are proposals; report citations and standards status remain unverified.
'''}
bundle={'schema':'claude-obsidian.transaction.v1','operation_id':'ingest-security-token-report9-20260909','operation_type':'ingest','expected_hashes':{},'writes':[],'address_requests':[],'source_manifest_updates':{raw:{'pages':pages,'sha256':sha,'source_id':legacy,'status':'ingested; S2 proposed asset-transformation analysis; external citations unverified'}}}
def write(path,content):
 old=(R/path).read_bytes() if (R/path).exists() else None
 bundle['expected_hashes'][path]=H(old) if old is not None else None
 bundle['writes'].append({'path':path,'mode':'replace' if old is not None else 'create','content':content,'sha256':H(content.encode())})
for path,addition in additions.items():
 text=(R/path).read_text(); front,body=text[4:].split('---',1)
 front=re.sub(r'^updated:.*$',f'updated: {day}',front,flags=re.M)
 front=re.sub(r'^updated_at:.*$',f'updated_at: {now}',front,flags=re.M)
 front=front.replace('sources:\n',f'sources:\n  - {legacy}\n',1)
 write(path,'---\n'+front+'---'+body.rstrip()+addition)
reader=csv.DictReader(io.StringIO((R/'evidence/source-inventory.csv').read_text()));rows=list(reader);fields=reader.fieldnames
record=dict(zip(fields,[legacy,'Security Token Standards Across EVM and Non-EVM Ecosystems (supplied report 9)','user-supplied Markdown','user-supplied report; opaque citation map unavailable','2026-09-09 (report assertion)',now,'','secondary descriptive synthesis','asset/claim transformations, policy paths and agenda design','S2','not reproduced','medium',raw,sha,'Full 565-line report read. Unknown author. Opaque citations and two unavailable companion catalogs; no independent status/legal/deployment verification. SRC-0109 belongs to syntax research branch.']))
sl['legacy_source_ids'][legacy]=sid;sl['sources'][sid]={'origin':{'kind':'file','locator':raw},'title':record['title'],'content_kind':'document','authority':'secondary','review_status':'active','content_sha256':sha,'ingested_at':day,'retrieved_at':day,'refresh_due':'2026-10-09','pages':pages,'independence_key':'user-supplied-defi-research-reports','legacy_records':[record],'notes':'Active as inspected source material only. External claims unverified; not independent corroboration of underlying standards.'};sl['generated_at']=now
texts={943:('The report separates financial lifecycle and entitlement from identity, transfer control and chain enforcement.',pages[0],'Lines 317–385 and 539–555.'),944:('A bounded asset/claim/encumbrance model with operation-specific transformation rules is a proposed fit for the existing Moriarty roadmap.',pages[0],'Lines 317–473; proposed design crosswalk in deliverables/security-token-transformations-2026-09-09/DESIGN-IMPLICATIONS.md.'),945:('Operation-specific policy checks and scoped exceptions must cover wrapper, liquidation, recovery and migration paths in the proposed security-token conformance model.',pages[1],'Lines 365–413 and 447–473.')}
for n,(text,path,locator) in texts.items():
 cl['claims'][f'clm-moriarty-{n:04d}']={'text':text,'location':{'path':path,'anchor':None},'risk':'normal','assessment':'provisional','confidence':'medium','reviewed_at':day,'evidence':[{'source_id':sid,'relation':'supports','locator':locator}],'notes':f'Legacy CLM-{n:04d}; S2 source interpretation/proposed design only; no implementation, legal, proof or network acceptance.'}
 ci['claims'][f'CLM-{n:04d}']=[path,'wiki/index.md','wiki/log.md']
cl['generated_at']=now
for path,obj in [('wiki/meta/ledgers/source-ledger.json',sl),('wiki/meta/ledgers/claim-ledger.json',cl),('wiki/meta/legacy-claim-index.json',ci)]:write(path,json.dumps(obj,indent=2,ensure_ascii=False)+'\n')
(D/'ingest-bundle.json').write_text(json.dumps(bundle,indent=2,ensure_ascii=False)+'\n')
s=io.StringIO();w=csv.DictWriter(s,fieldnames=fields,lineterminator='\n');w.writeheader();w.writerows(rows+[record]);(D/'source-inventory.after.csv').write_text(s.getvalue());(D/'source-inventory-precondition.json').write_text(json.dumps({'path':'evidence/source-inventory.csv','before_sha256':H((R/'evidence/source-inventory.csv').read_bytes()),'after_sha256':H(s.getvalue().encode()),'reason':'Repository inventory is outside portable core wiki/raw write scope; append after successful ingest using exact hash precondition.'},indent=2)+'\n')
print('Drafted',len(bundle['writes']),'writes, source',legacy,'claims CLM-0943–CLM-0945')
